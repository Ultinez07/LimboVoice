import { useState, useEffect } from 'react';
import { Link } from 'react-router-dom';
import './Dashboard.css';

export function Dashboard() {
    const [stats, setStats] = useState({
        recordingsToday: 0,
        totalWords: 0,
        hotkeyConfigured: 'Alt+Space'
    });

    return (
        <div className="dashboard">
            <div className="dashboard-header">
                <h1>🎙️ Limbo Voice</h1>
                <p className="tagline">Universal Voice Dictation for Windows</p>
            </div>

            <div className="status-card">
                <div className="status-indicator active"></div>
                <div className="status-text">
                    <h2>Ready to Dictate</h2>
                    <p>Press <kbd>{stats.hotkeyConfigured}</kbd> anywhere to start recording</p>
                </div>
            </div>

            <div className="stats-grid">
                <div className="stat-card">
                    <div className="stat-icon">📊</div>
                    <div className="stat-content">
                        <h3>{stats.recordingsToday}</h3>
                        <p>Recordings Today</p>
                    </div>
                </div>

                <div className="stat-card">
                    <div className="stat-icon">✍️</div>
                    <div className="stat-content">
                        <h3>{stats.totalWords.toLocaleString()}</h3>
                        <p>Words Transcribed</p>
                    </div>
                </div>
            </div>

            <div className="quick-actions">
                <Link to="/settings" className="action-button primary">
                    ⚙️ Settings
                </Link>
                <button className="action-button secondary">
                    🧪 Test Recording
                </button>
            </div>

            <div className="info-section">
                <h3>✨ Features</h3>
                <ul className="feature-list">
                    <li>✅ 100% Free & Offline</li>
                    <li>✅ No API Keys Required</li>
                    <li>✅ Privacy-First (Audio never leaves your PC)</li>
                    <li>✅ Works in Any Application</li>
                </ul>
            </div>
        </div>
    );
}
