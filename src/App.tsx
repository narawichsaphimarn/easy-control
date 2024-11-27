import {useEffect, useState} from "react";
import "./App.css";
import { Navigate } from "./Navigate";
import {invoke} from "@tauri-apps/api/core";

function App() {
  // const [greetMsg, setGreetMsg] = useState("");
  const [role, setRole] = useState<string>("");
  useEffect(() => {
    const link = document.createElement("link");
    link.rel = "stylesheet";
    link.href =
      "https://cdnjs.cloudflare.com/ajax/libs/font-awesome/4.7.0/css/font-awesome.min.css";
    document.head.appendChild(link);
    getRole().catch(error => {
      console.error("Failed to get role:", error);
    });
  }, []);

  const getRole = async () => {
    setRole(await invoke("get_role"))
  }
  // async function greet() {
  //   // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  //   setGreetMsg(await invoke("greet", { name }));
  // }

  return (
    <main className="container">
      <Navigate role={role} />
    </main>
  );
}

export default App;
