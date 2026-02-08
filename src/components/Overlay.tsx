import React from 'react';
import { motion, AnimatePresence } from 'framer-motion';

interface OverlayProps {
    isRecording: boolean;
    status: 'idle' | 'recording' | 'transcribing' | 'success' | 'error';
    transcript?: string;
}

export const Overlay: React.FC<OverlayProps> = ({ isRecording, status, transcript }) => {
    return (
        <AnimatePresence>
            {(isRecording || status !== 'idle') && (
                <motion.div
                    initial={{ opacity: 0, scale: 0.9, y: 20 }}
                    animate={{ opacity: 1, scale: 1, y: 0 }}
                    exit={{ opacity: 0, scale: 0.9, y: 20 }}
                    transition={{ type: 'spring', damping: 20, stiffness: 300 }}
                    className="glass-panel main-overlay"
                >
                    <div className="content-wrapper">
                        {/* Pulsating Ring */}
                        <div className={`status-ring ${status}`}>
                            <div className="ring-inner">
                                {status === 'recording' ? (
                                    <MicIcon className="mic-icon pulse" />
                                ) : status === 'transcribing' ? (
                                    <LoadingIcon className="loading-icon spin" />
                                ) : (
                                    <CheckIcon className="check-icon" />
                                )}
                            </div>
                        </div>

                        <div className="text-section">
                            <span className="status-badge">
                                {status.toUpperCase()}
                            </span>
                            <h2 className="transcript-preview">
                                {status === 'recording' ? 'Hearing...' :
                                    status === 'transcribing' ? 'Thinking...' :
                                        transcript || 'Ready to type'}
                            </h2>
                        </div>
                    </div>

                    {/* Bottom Glow bar */}
                    <motion.div
                        className="bottom-glow"
                        animate={{
                            opacity: status === 'recording' ? [0.4, 0.8, 0.4] : 0
                        }}
                        transition={{ duration: 2, repeat: Infinity }}
                    />
                </motion.div>
            )}
        </AnimatePresence>
    );
};

// Simple SVG Icons
const MicIcon = ({ className }: { className?: string }) => (
    <svg className={className} width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
        <path d="M12 2a3 3 0 0 0-3 3v7a3 3 0 0 0 6 0V5a3 3 0 0 0-3-3z" />
        <path d="M19 10v1a7 7 0 0 1-14 0v-1" />
        <line x1="12" y1="19" x2="12" y2="22" />
        <line x1="8" y1="22" x2="16" y2="22" />
    </svg>
);

const LoadingIcon = ({ className }: { className?: string }) => (
    <svg className={className} width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="3" strokeLinecap="round" strokeLinejoin="round">
        <path d="M21 12a9 9 0 1 1-6.219-8.56" />
    </svg>
);

const CheckIcon = ({ className }: { className?: string }) => (
    <svg className={className} width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="3" strokeLinecap="round" strokeLinejoin="round">
        <polyline points="20 6 9 17 4 12" />
    </svg>
);
