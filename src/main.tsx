import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./index.css";

// Show window after React content loads (prevents white flash)
import { getCurrentWindow } from '@tauri-apps/api/window';

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);

// Hide loading screen and show window after React loads
setTimeout(() => {
  const loadingScreen = document.getElementById('loading-screen');
  if (loadingScreen) {
    loadingScreen.style.opacity = '0';
    loadingScreen.style.transition = 'opacity 0.3s ease';
    setTimeout(() => loadingScreen.remove(), 300);
  }
  getCurrentWindow().show();
}, 100);
