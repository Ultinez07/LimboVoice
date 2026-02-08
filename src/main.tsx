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

// Show window after initial render
getCurrentWindow().show();
