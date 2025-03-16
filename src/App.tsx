import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { listen, Event } from "@tauri-apps/api/event";

interface IAddCredits {
  credits: number;
}

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");
  const [serverStatus, setServerStatus] = useState("offline");
  const [coin, setCoin] = useState(0);

  useEffect(() => {
    const unlistenRegister = listen("register_request", (event) => {
      console.log("Received a received request", event.payload);
      setServerStatus("Received a valid request");
    });

    const unlistenAddTime = listen(
      "addtime_handler",
      (event: Event<IAddCredits>) => {
        console.log("Received add time request", event.payload);
        let credits = event.payload.credits;
        setCoin(credits);
      },
    );

    return () => {
      unlistenRegister.then((unlistenFn) => unlistenFn());
      unlistenAddTime.then((unlistenFn) => unlistenFn());
    };
  }, []);

  async function greet() {
    setGreetMsg(await invoke("greet", { name }));
  }

  return (
    <main className="container">
      <h1>Welcome to Tauri + React</h1>

      <div className="row">
        <h2>{serverStatus}</h2>
        <h2>Inserted PHP {coin}</h2>
      </div>
      <p>Click on the Tauri, Vite, and React logos to learn more.</p>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button type="submit">Greet</button>
      </form>
      <p>{greetMsg}</p>
    </main>
  );
}

export default App;
