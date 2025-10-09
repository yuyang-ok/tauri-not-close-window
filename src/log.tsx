import React, { useEffect } from "react";
import ReactDOM from "react-dom/client";
import { Channel, invoke } from "@tauri-apps/api/core";

function Log() {
  useEffect(() => {
    const onEvent = new Channel<string>();
    onEvent.onmessage = (message) => {
      console.log(`got download event `, message);
    };
    invoke("download", {
      onEvent,
    });
  }, []);

  return <div>123 </div>;
}

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <Log />
  </React.StrictMode>
);
