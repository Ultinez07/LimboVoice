"""
Limbo Voice - Python Edition
A lightweight voice dictation tool with Alt+Space hotkey
"""

import tkinter as tk
from tkinter import ttk
import keyboard
import pyaudio
import wave
import threading
import tempfile
import os
from pynput.keyboard import Controller, Key
import speech_recognition as sr
from datetime import datetime

class LimboVoice:
    def __init__(self):
        self.root = tk.Tk()
        self.root.title("Limbo Voice")
        self.root.overrideredirect(True)  # Remove window decorations
        self.root.attributes('-topmost', True)  # Always on top
        self.root.attributes('-alpha', 0.95)  # Slight transparency
        
        # Window size and position
        window_width = 400
        window_height = 200
        screen_width = self.root.winfo_screenwidth()
        screen_height = self.root.winfo_screenheight()
        x = (screen_width - window_width) // 2
        y = (screen_height - window_height) // 2
        self.root.geometry(f'{window_width}x{window_height}+{x}+{y}')
        
        # Hide window initially
        self.root.withdraw()
        
        # Audio settings
        self.is_recording = False
        self.audio_frames = []
        self.audio = pyaudio.PyAudio()
        
        # Speech recognizer
        self.recognizer = sr.Recognizer()
        
        # Keyboard controller for typing
        self.keyboard_controller = Controller()
        
        # Setup UI
        self.setup_ui()
        
        # Register global hotkey
        keyboard.add_hotkey('alt+space', self.toggle_recording)
        
        print("Limbo Voice is running! Press Alt+Space to start dictating.")
        
    def setup_ui(self):
        # Main frame with dark background
        main_frame = tk.Frame(
            self.root,
            bg='#1a1a2e',
            highlightbackground='#00ff9f',
            highlightthickness=2
        )
        main_frame.pack(fill='both', expand=True, padx=10, pady=10)
        
        # Status indicator (circle)
        self.status_canvas = tk.Canvas(
            main_frame,
            width=80,
            height=80,
            bg='#1a1a2e',
            highlightthickness=0
        )
        self.status_canvas.pack(pady=20)
        
        # Draw initial circle
        self.status_circle = self.status_canvas.create_oval(
            10, 10, 70, 70,
            fill='#00ff9f',
            outline='#00ff9f',
            width=2
        )
        
        # Microphone icon (simplified)
        self.mic_icon = self.status_canvas.create_text(
            40, 40,
            text='🎙️',
            font=('Arial', 32),
            fill='#1a1a2e'
        )
        
        # Status label
        self.status_label = tk.Label(
            main_frame,
            text='READY',
            font=('Arial', 14, 'bold'),
            bg='#1a1a2e',
            fg='#00ff9f'
        )
        self.status_label.pack(pady=5)
        
        # Transcript label
        self.transcript_label = tk.Label(
            main_frame,
            text='Press Alt+Space to speak',
            font=('Arial', 10),
            bg='#1a1a2e',
            fg='#a0a0a0',
            wraplength=350
        )
        self.transcript_label.pack(pady=10)
        
    def toggle_recording(self):
        if not self.is_recording:
            self.start_recording()
        else:
            self.stop_recording()
    
    def start_recording(self):
        self.is_recording = True
        self.audio_frames = []
        
        # Show window
        self.root.deiconify()
        
        # Update UI
        self.status_label.config(text='LISTENING...', fg='#ff3366')
        self.status_canvas.itemconfig(self.status_circle, fill='#ff3366', outline='#ff3366')
        self.transcript_label.config(text='Speak now...', fg='#ffffff')
        
        # Start recording in background thread
        self.recording_thread = threading.Thread(target=self.record_audio)
        self.recording_thread.start()
    
    def record_audio(self):
        # Audio recording parameters
        CHUNK = 1024
        FORMAT = pyaudio.paInt16
        CHANNELS = 1
        RATE = 16000
        
        stream = self.audio.open(
            format=FORMAT,
            channels=CHANNELS,
            rate=RATE,
            input=True,
            frames_per_buffer=CHUNK
        )
        
        print("Recording started...")
        
        while self.is_recording:
            data = stream.read(CHUNK, exception_on_overflow=False)
            self.audio_frames.append(data)
        
        print("Recording stopped.")
        stream.stop_stream()
        stream.close()
    
    def stop_recording(self):
        self.is_recording = False
        
        # Update UI
        self.status_label.config(text='PROCESSING...', fg='#ffaa00')
        self.status_canvas.itemconfig(self.status_circle, fill='#ffaa00', outline='#ffaa00')
        self.transcript_label.config(text='Transcribing your speech...', fg='#ffffff')
        
        # Process audio in background
        threading.Thread(target=self.transcribe_and_type).start()
    
    def transcribe_and_type(self):
        if not self.audio_frames:
            self.hide_window()
            return
        
        # Save audio to temporary file
        temp_file = tempfile.NamedTemporaryFile(delete=False, suffix='.wav')
        wf = wave.open(temp_file.name, 'wb')
        wf.setnchannels(1)
        wf.setsampwidth(self.audio.get_sample_size(pyaudio.paInt16))
        wf.setframerate(16000)
        wf.writeframes(b''.join(self.audio_frames))
        wf.close()
        
        try:
            # Transcribe using Google Speech Recognition (free)
            with sr.AudioFile(temp_file.name) as source:
                audio_data = self.recognizer.record(source)
                text = self.recognizer.recognize_google(audio_data)
                
                # Update UI with success
                self.status_label.config(text='SUCCESS!', fg='#00ff9f')
                self.status_canvas.itemconfig(self.status_circle, fill='#00ff9f', outline='#00ff9f')
                self.transcript_label.config(text=f'"{text}"', fg='#ffffff')
                
                # Hide window after 2 seconds
                self.root.after(2000, self.hide_window)
                
                # Type the text
                self.root.after(100, lambda: self.type_text(text))
                
        except sr.UnknownValueError:
            self.status_label.config(text='ERROR', fg='#ff3366')
            self.transcript_label.config(text='Could not understand audio', fg='#ff3366')
            self.root.after(2000, self.hide_window)
            
        except sr.RequestError as e:
            self.status_label.config(text='ERROR', fg='#ff3366')
            self.transcript_label.config(text=f'Service error: {e}', fg='#ff3366')
            self.root.after(2000, self.hide_window)
        
        finally:
            # Clean up temp file
            os.unlink(temp_file.name)
    
    def type_text(self, text):
        """Type the transcribed text at cursor position"""
        # Small delay to ensure focus
        import time
        time.sleep(0.1)
        
        # Type each character
        self.keyboard_controller.type(text)
    
    def hide_window(self):
        self.root.withdraw()
        # Reset UI
        self.status_label.config(text='READY', fg='#00ff9f')
        self.status_canvas.itemconfig(self.status_circle, fill='#00ff9f', outline='#00ff9f')
        self.transcript_label.config(text='Press Alt+Space to speak', fg='#a0a0a0')
    
    def run(self):
        try:
            self.root.mainloop()
        except KeyboardInterrupt:
            self.cleanup()
    
    def cleanup(self):
        self.audio.terminate()
        self.root.destroy()

if __name__ == "__main__":
    app = LimboVoice()
    app.run()
