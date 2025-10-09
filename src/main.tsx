import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import { WebviewWindow } from "@tauri-apps/api/webviewWindow";

const newWebview = new WebviewWindow("log", {
  url: "log.html",
  width: 800,
  height: 600,
});

newWebview.once("tauri://created", () => {
  console.log("Webview window created successfully!");
});
newWebview.once("tauri://error", (e) => {
  console.error("Error creating webview window:", e);
});

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
