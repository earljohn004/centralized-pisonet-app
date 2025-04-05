import { invoke } from "@tauri-apps/api/core";
import { useSettingStore } from "../store/Settings";
import { useRef, useState } from "react";

export const MainWindow = () => {
  const serverStatus = useSettingStore((state) => state.serverStatus);
  const coin = useSettingStore((state) => state.coin);
  const remainingTime = useSettingStore((state) => state.remainingTime);
  const timerDone = useSettingStore((state) => state.timerDone);
  const {
    serialNumber: licenseSerialNumber,
    authorized: isAuthorized,
    emailAddress: licenseEmailAddress,
  } = useSettingStore((state) => state.licenseInformation);

  const serialNumberRef = useRef<HTMLInputElement>(null);
  const emailAddressRef = useRef<HTMLInputElement>(null);
  const [authorized, setAuthorized] = useState(false);

  async function authorize() {
    const serialNumber = serialNumberRef.current?.value || "";
    const emailAddress = emailAddressRef.current?.value || "";
    setAuthorized(
      await invoke<boolean>("authorize", { serialNumber, emailAddress }),
    );
  }

  return (
    <>
      <h1>Welcome MPG Cafe</h1>
      <h2>{serverStatus}</h2>
      <h2>Serial Number: {licenseSerialNumber}</h2>
      <h2>Owner Address: {licenseEmailAddress}</h2>
      <h2>License Authorized: {isAuthorized ? "True" : "False"}</h2>

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
    </>
  );
};
