import { useEffect, useState } from "react";
import "./HearingIndicator.css";

interface HearingIndicatorProps {
    isRecording: boolean;
    status: string;
}

export default function HearingIndicator({ isRecording, status }: HearingIndicatorProps) {
    const [visible, setVisible] = useState(false);

    useEffect(() => {
        if (isRecording) {
            setVisible(true);
        } else {
            // Keep visible for 2 seconds after recording stops
            const timer = setTimeout(() => setVisible(false), 2000);
            return () => clearTimeout(timer);
        }
    }, [isRecording]);

    if (!visible) return null;

    return (
        <div className={`hearing-indicator ${isRecording ? "recording" : "processing"}`}>
            <div className="indicator-content">
                <div className="logo-container">
                    {/* Animated microphone icon */}
                    <svg
                        className="mic-icon"
                        viewBox="0 0 24 24"
                        fill="none"
                        xmlns="http://www.w3.org/2000/svg"
                    >
                        <path
                            d="M12 15C13.6569 15 15 13.6569 15 12V6C15 4.34315 13.6569 3 12 3C10.3431 3 9 4.34315 9 6V12C9 13.6569 10.3431 15 12 15Z"
                            stroke="currentColor"
                            strokeWidth="2"
                            strokeLinecap="round"
                            strokeLinejoin="round"
                        />
                        <path
                            d="M19 12C19 15.866 15.866 19 12 19M12 19C8.13401 19 5 15.866 5 12M12 19V23M8 23H16"
                            stroke="currentColor"
                            strokeWidth="2"
                            strokeLinecap="round"
                            strokeLinejoin="round"
                        />
                    </svg>

                    {/* Pulse rings when recording */}
                    {isRecording && (
                        <>
                            <div className="pulse-ring ring-1"></div>
                            <div className="pulse-ring ring-2"></div>
                            <div className="pulse-ring ring-3"></div>
                        </>
                    )}
                </div>

                <div className="status-text">{status}</div>
            </div>
        </div>
    );
}
