import { useState } from "react";
import { Overlay } from "./components/Overlay";
import "./App.css";

function App() {
  const [isRecording, setIsRecording] = useState(false);
  const [status, setStatus] = useState<'idle' | 'recording' | 'transcribing' | 'success' | 'error'>('idle');
  const [transcript, setTranscript] = useState("");

  // Demo toggle for visual testing
  const toggleRecording = () => {
    if (!isRecording) {
      setIsRecording(true);
      setStatus('recording');
    } else {
      setStatus('transcribing');
      // Simulate finish after 1.5s
      setTimeout(() => {
        setStatus('success');
        setTranscript("Hello, this is a beautiful dictation demo!");
        setTimeout(() => {
          setIsRecording(false);
          setStatus('idle');
          setTranscript("");
        }, 3000);
      }, 1500);
    }
  };

  return (
    <main className="container">
      <Overlay
        isRecording={isRecording}
        status={status}
        transcript={transcript}
      />

      {/* 
          This button is for manual testing while we wait for 
          Rust Build Tools to finish. 
      */}
      <div style={{ position: 'fixed', bottom: 20, right: 20, opacity: 0.2 }}>
        <button onClick={toggleRecording}>
          {isRecording ? "Stop Talk" : "Start Talk"}
        </button>
      </div>
    </main>
  );
}

export default App;
