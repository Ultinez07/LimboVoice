use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Emitter};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut};

#[derive(Clone, serde::Serialize)]
struct RecordingState {
    is_recording: bool,
    status: String,
}

struct AppState {
    recording: Arc<Mutex<bool>>,
    audio_buffer: Arc<Mutex<Vec<f32>>>,
}

// Command to start recording audio
#[tauri::command]
async fn start_recording(app: AppHandle, state: tauri::State<'_, AppState>) -> Result<String, String> {
    let mut is_recording = state.recording.lock().unwrap();
    *is_recording = true;
    
    // Clear previous audio buffer
    let mut buffer = state.audio_buffer.lock().unwrap();
    buffer.clear();
    
    // Emit event to frontend
    app.emit("recording-state", RecordingState {
        is_recording: true,
        status: "Listening...".to_string(),
    }).map_err(|e| e.to_string())?;
    
    // Start audio capture in background
    let audio_buffer = state.audio_buffer.clone();
    let recording = state.recording.clone();
    let app_clone = app.clone();
    
    std::thread::spawn(move || {
        if let Err(e) = capture_audio(audio_buffer, recording, app_clone) {
            eprintln!("Audio capture error: {}", e);
        }
    });
    
    Ok("Recording started".to_string())
}

// Command to stop recording and transcribe
#[tauri::command]
async fn stop_recording(app: AppHandle, state: tauri::State<'_, AppState>) -> Result<String, String> {
    let mut is_recording = state.recording.lock().unwrap();
    *is_recording = false;
    
    app.emit("recording-state", RecordingState {
        is_recording: false,
        status: "Complete!".to_string(),
    }).map_err(|e| e.to_string())?;
    
    Ok("Recording stopped".to_string())
}

// Process audio chunks in real-time for live transcription
async fn process_audio_chunks(
    buffer: Arc<Mutex<Vec<f32>>>,
    is_recording: Arc<Mutex<bool>>,
    app: AppHandle,
) {
    const CHUNK_SIZE_SECONDS: f32 = 2.0;  // Process every 2 seconds
    const SAMPLE_RATE: u32 = 16000;
    let chunk_samples = (CHUNK_SIZE_SECONDS * SAMPLE_RATE as f32) as usize;
    
    let mut last_processed_index = 0;
    
    while *is_recording.lock().unwrap() {
        // Wait a bit before checking for new audio
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        
        let buffer_len = buffer.lock().unwrap().len();
        
        // Check if we have enough new samples for a chunk
        if buffer_len >= last_processed_index + chunk_samples {
            // Extract chunk
            let chunk: Vec<f32> = {
                let buf = buffer.lock().unwrap();
                buf[last_processed_index..last_processed_index + chunk_samples].to_vec()
            };
            
            last_processed_index += chunk_samples;
            
            // Save chunk to temp file
            let temp_path = std::env::temp_dir().join(format!("limbo_chunk_{}.wav", last_processed_index));
            if let Ok(()) = save_audio_chunk_to_wav(&chunk, &temp_path) {
                // Transcribe this chunk
                match transcribe_with_whisper(&temp_path).await {
                    Ok(text) => {
                        if !text.trim().is_empty() {
                            // Type the transcribed text immediately
                            if let Err(e) = type_text(&text) {
                                eprintln!("Failed to type text chunk: {}", e);
                            }
                            
                            // Emit event to UI
                            let _ = app.emit("chunk-transcribed", text);
                        }
                    }
                    Err(e) => {
                        eprintln!("Chunk transcription error: {}", e);
                    }
                }
                
                // Clean up chunk file
                let _ = std::fs::remove_file(&temp_path);
            }
        }
    }
}

// Capture audio from microphone
fn capture_audio(
    buffer: Arc<Mutex<Vec<f32>>>,
    is_recording: Arc<Mutex<bool>>,
    _app: AppHandle,
) -> Result<(), Box<dyn std::error::Error>> {
    use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
    
    let host = cpal::default_host();
    let device = host.default_input_device()
        .ok_or("No input device available")?;
    
    let config = device.default_input_config()?;
    
    let buffer_clone = buffer.clone();
    let is_recording_clone = is_recording.clone();
    
    let stream = match config.sample_format() {
        cpal::SampleFormat::F32 => device.build_input_stream(
            &config.into(),
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                let recording = is_recording_clone.lock().unwrap();
                if *recording {
                    let mut buf = buffer_clone.lock().unwrap();
                    buf.extend_from_slice(data);
                }
            },
            |err| eprintln!("Stream error: {}", err),
            None,
        )?,
        _ => return Err("Unsupported sample format".into()),
    };
    
    stream.play()?;
    
    // Keep stream alive while recording
    while *is_recording.lock().unwrap() {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    
    Ok(())
}

// Save audio buffer to WAV file
fn save_audio_to_wav(buffer: &[f32], path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 16000,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };
    
    let mut writer = hound::WavWriter::create(path, spec)?;
    for &sample in buffer {
        writer.write_sample(sample)?;
    }
    writer.finalize()?;
    
    Ok(())
}

// Save audio chunk to WAV file (for live streaming)
fn save_audio_chunk_to_wav(buffer: &[f32], path: &std::path::Path) -> Result<(), String> {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 16000,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };
    
    let mut writer = hound::WavWriter::create(path, spec).map_err(|e| e.to_string())?;
    for &sample in buffer {
        writer.write_sample(sample).map_err(|e| e.to_string())?;
    }
    writer.finalize().map_err(|e| e.to_string())?;
    
    Ok(())
}


// Transcribe audio using local Whisper Turbo model
async fn transcribe_with_whisper(audio_path: &std::path::Path) -> Result<String, String> {
    use whisper_rs::{WhisperContext, WhisperContextParameters, FullParams, SamplingStrategy};
    
    // Get model path from bundled resources
    let model_path = std::env::current_exe()
        .map_err(|e| format!("Failed to get exe path: {}", e))?
        .parent()
        .ok_or("Failed to get parent directory")?
        .join("resources")
        .join("ggml-small.en.bin");
    
    if !model_path.exists() {
        return Err(format!(
            "Whisper model not found at: {}\n\nThis build is missing the AI model. Please download Build #14 or later from GitHub Releases.",
            model_path.display()
        ));
    }
    
    // Load Whisper model
    let ctx = WhisperContext::new_with_params(
        model_path.to_str().unwrap(),
        WhisperContextParameters::default()
    ).map_err(|e| format!("Failed to load Whisper model: {}", e))?;
    
    // Create transcription parameters
    let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
    params.set_language(Some("en"));  // English only for faster processing
    params.set_translate(false);
    params.set_print_progress(false);
    params.set_print_realtime(false);
    params.set_print_timestamps(false);
    
    // Create state for transcription
    let mut state = ctx.create_state()
        .map_err(|e| format!("Failed to create Whisper state: {}", e))?;
    
    // Read and convert audio to required format
    let audio_data = read_wav_to_samples(audio_path)?;
    
    // Perform transcription
    state.full(params, &audio_data)
        .map_err(|e| format!("Transcription failed: {}", e))?;
    
    // Get transcribed text
    let num_segments = state.full_n_segments()
        .map_err(|e| format!("Failed to get segments: {}", e))?;
    
    let mut transcribed_text = String::new();
    for i in 0..num_segments {
        let segment = state.full_get_segment_text(i)
            .map_err(|e| format!("Failed to get segment {}: {}", i, e))?;
        transcribed_text.push_str(&segment);
        transcribed_text.push(' ');
    }
    
    Ok(transcribed_text.trim().to_string())
}

// Convert WAV file to f32 samples for Whisper
fn read_wav_to_samples(path: &std::path::Path) -> Result<Vec<f32>, String> {
    let mut reader = hound::WavReader::open(path)
        .map_err(|e| format!("Failed to open WAV: {}", e))?;
    
    let samples: Vec<f32> = reader.samples::<f32>()
        .map(|s| s.unwrap_or(0.0))
        .collect();
    
    Ok(samples)
}

// Type text using enigo
fn type_text(text: &str) -> Result<(), String> {
    use enigo::{Enigo, Settings, Keyboard};
    
    let mut enigo = Enigo::new(&Settings::default()).map_err(|e| e.to_string())?;
    
    // Small delay to ensure target app is focused
    std::thread::sleep(std::time::Duration::from_millis(100));
    
    // Type the text
    enigo.text(text).map_err(|e| e.to_string())?;
    
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app_state = AppState {
        recording: Arc::new(Mutex::new(false)),
        audio_buffer: Arc::new(Mutex::new(Vec::new())),
    };
    
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![start_recording, stop_recording])
        .setup(|app| {
            // Create system tray icon
            use tauri::menu::{Menu, MenuItemBuilder};
            use tauri::tray::{TrayIconBuilder, TrayIconEvent};
            
            let quit = MenuItemBuilder::with_id("quit", "Quit Limbo Voice").build(app)?;
            let menu = Menu::with_items(app, &[&quit])?;
            
            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .menu_on_left_click(false)
                .on_menu_event(|app, event| {
                    if event.id == "quit" {
                        app.exit(0);
                    }
                })
                .on_tray_icon_event(|_tray, event| {
                    if let TrayIconEvent::Click { button, .. } = event {
                        if button == tauri::tray::MouseButton::Left {
                            println!("Limbo Voice is running. Press Alt+Space to dictate!");
                        }
                    }
                })
                .build(app)?;
            
            // Register global shortcut Alt+Space
            let handle = app.handle().clone();
            let shortcut = Shortcut::new(Some(tauri_plugin_global_shortcut::Modifiers::ALT), tauri_plugin_global_shortcut::Code::Space);
            
            app.global_shortcut().on_shortcut(shortcut, move |_app, _shortcut, _event| {
                // Toggle recording
                // This will be handled by the frontend
                let _ = handle.emit("hotkey-pressed", ());
            })?;
            
            app.global_shortcut().register(shortcut)?;
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
