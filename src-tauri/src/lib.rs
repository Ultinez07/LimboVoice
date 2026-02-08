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
        status: "Processing...".to_string(),
    }).map_err(|e| e.to_string())?;
    
    // Get audio buffer
    let buffer = state.audio_buffer.lock().unwrap();
    
    // Save audio to temp file
    let temp_path = std::env::temp_dir().join("limbo_voice_temp.wav");
    if let Err(e) = save_audio_to_wav(&buffer, &temp_path) {
        return Err(format!("Failed to save audio: {}", e));
    }
    
    // TODO: Integrate Whisper model here
    // For now, returning placeholder text
    let transcribed_text = "Whisper transcription will go here".to_string();
    
    // Simulate typing the text
    type_text(&transcribed_text)?;
    
    app.emit("recording-state", RecordingState {
        is_recording: false,
        status: "Complete!".to_string(),
    }).map_err(|e| e.to_string())?;
    
    Ok(transcribed_text)
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
