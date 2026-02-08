import { useState } from 'react';
import { Link } from 'react-router-dom';
import './Settings.css';

export function Settings() {
    const [hotkey, setHotkey] = useState('Alt+Space');
    const [isRecordingHotkey, setIsRecordingHotkey] = useState(false);

    const startHotkeyRecording = () => {
        setIsRecordingHotkey(true);
        // TODO: Implement hotkey capture
    };

    return (
        <div className="settings">
            <div className="settings-header">
                <Link to="/" className="back-button">← Back to Dashboard</Link>
                <h1>⚙️ Settings</h1>
            </div>

            <div className="settings-section">
                <h2>Hotkey Configuration</h2>
                <p className="section-description">Customize your voice dictation hotkey. Supports keyboard and mouse buttons.</p>

                <div className="hotkey-picker">
                    <div className="current-hotkey">
                        <label>Current Hotkey:</label>
                        <div className="hotkey-display">{hotkey}</div>
                    </div>

                    <button
                        className={`record-hotkey-btn ${isRecordingHotkey ? 'recording' : ''}`}
                        onClick={startHotkeyRecording}
                    >
                        {isRecordingHotkey ? '⏺ Press any key combination...' : '🎯 Change Hotkey'}
                    </button>

                    <div className="hotkey-suggestions">
                        <p>Popular combinations:</p>
                        <div className="suggestion-chips">
                            <button className="chip" onClick={() => setHotkey('Alt+Space')}>Alt+Space</button>
                            <button className="chip" onClick={() => setHotkey('Ctrl+Shift+V')}>Ctrl+Shift+V</button>
                            <button className="chip" onClick={() => setHotkey('Mouse4')}>Mouse4</button>
                            <button className="chip" onClick={() => setHotkey('F9')}>F9</button>
                        </div>
                    </div>
                </div>
            </div>

            <div className="settings-section">
                <h2>Audio Settings</h2>
                <div className="setting-row">
                    <label>Microphone:</label>
                    <select className="select-input">
                        <option>Default Microphone</option>
                    </select>
                </div>

                <button className="test-button">🎤 Test Microphone</button>
            </div>

            <div className="settings-section">
                <h2>Behavior</h2>

                <div className="setting-row checkbox-row">
                    <input type="checkbox" id="startup" defaultChecked />
                    <label htmlFor="startup">Start with Windows</label>
                </div>

                <div className="setting-row checkbox-row">
                    <input type="checkbox" id="tray" defaultChecked />
                    <label htmlFor="tray">Minimize to system tray</label>
                </div>
            </div>

            <div className="settings-footer">
                <button className="save-button">💾 Save Changes</button>
            </div>
        </div>
    );
}
