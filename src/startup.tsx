import React from "react";
import ReactDOM from "react-dom/client";
import StartupApp from "./StartupApp";
import "./styles.css";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <StartupApp />
  </React.StrictMode>,
);
