import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { listen, Event } from "@tauri-apps/api/event";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");
  const [serverStatus, setServerStatus] = useState("offline");
  const [coin, setCoin] = useState(0);
  const [remainingTime, setRemainingTime] = useState(0);
  const [timerDone, setTimerDone] = useState(false);

  useEffect(() => {
    const unlistenRegister = listen("register_request", (event) => {
      console.log("Received a received request", event.payload);
      setServerStatus("Received a valid request");
    });

    const unlistenAddTime = listen(
      "addtime_handler",
      (event: Event<number>) => {
        console.log("Received add time request", event.payload);
        let credits = event.payload;
        setCoin(credits);
      },
    );
    // Listen for timer updates
    const unlistenTimerUpdate = listen(
      "timer_update",
      (event: Event<number>) => {
        setRemainingTime(event.payload); // Update the remaining time
      },
    );

    // Listen for the timer completion event
    const unlistenTimerDone = listen("timer_done", () => {
      setTimerDone(true); // Set the timer as done
    });

    return () => {
      unlistenRegister.then((unlistenFn) => unlistenFn());
      unlistenAddTime.then((unlistenFn) => unlistenFn());
      unlistenTimerUpdate.then((unlistenFn) => unlistenFn());
      unlistenTimerDone.then((unlistenFn) => unlistenFn());
    };
  }, []);

  async function greet() {
    setGreetMsg(await invoke("greet", { name }));
  }

  return (
    <main className="container">
      <h1>Welcome MPG Cafe</h1>
      <h2>{serverStatus}</h2>
      <h2>Inserted PHP {coin}</h2>

      {timerDone ? (
        <h3>Timer is done!</h3>
      ) : (
        <h3>Remaining Time: {remainingTime} seconds</h3>
      )}

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
        <button type="submit">Connect in rust</button>
      </form>
      <p>{greetMsg}</p>
    </main>
  );
}

export default App;
