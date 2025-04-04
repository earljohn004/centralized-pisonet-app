import { invoke } from "@tauri-apps/api/core";
import { useSettingStore } from "../SettingStore";
import { useRef, useState } from "react";

export const MainWindow = () => {
  const greetMsg = useSettingStore((state) => state.greetMsg);
  const serverStatus = useSettingStore((state) => state.serverStatus);
  const coin = useSettingStore((state) => state.coin);
  const remainingTime = useSettingStore((state) => state.remainingTime);
  const timerDone = useSettingStore((state) => state.timerDone);
  const setGreetMsg = useSettingStore((state) => state.setGreetMsg);
  const name = useSettingStore((state) => state.name);

  const serialNumberRef = useRef<HTMLInputElement>(null);
  const emailAddressRef = useRef<HTMLInputElement>(null);
  const [authorized, setAuthorized] = useState(false);

  async function greet() {
    setGreetMsg(await invoke("greet", { name }));
  }

  async function authorize() {
    const serialNumber = serialNumberRef.current?.value || "";
    const emailAddress = emailAddressRef.current?.value || "";
    setAuthorized(await invoke<(boolean)>("authorize", { serialNumber, emailAddress }));
  }

  return (
    <>
      <h1>Welcome MPG Cafe</h1>
      <h2>{serverStatus}</h2>
      <h2>Authorized: {authorized ? "Yes" : "No"}</h2>

      {timerDone ? (
        <>
          <h3>Insert Coin</h3>
        </>
      ) : (
        <>
          <h2>Inserted PHP {coin}</h2>
          <h3>Remaining Time: {remainingTime} seconds</h3>
        </>
      )}

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          authorize();
        }}
      >
        <input
          id="serial-input"
          ref={serialNumberRef}
          placeholder="Enter Serial Number..."
        />
        <input
          id="email-input"
          ref={emailAddressRef}
          placeholder="Enter Owner Address..."
        />
        <button type="submit">Connect in rust</button>
      </form>
      <p>{greetMsg}</p>
    </>
  );
};
