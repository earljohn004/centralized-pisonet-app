import { invoke } from "@tauri-apps/api/core";
import { useSettingStore } from "../SettingStore";

export const MainWindow = () => {
  const greetMsg = useSettingStore((state) => state.greetMsg);
  const setName = useSettingStore((state) => state.setName);
  const serverStatus = useSettingStore((state) => state.serverStatus);
  const coin = useSettingStore((state) => state.coin);
  const remainingTime = useSettingStore((state) => state.remainingTime);
  const timerDone = useSettingStore((state) => state.timerDone);
  const setGreetMsg = useSettingStore((state) => state.setGreetMsg);
  const name = useSettingStore((state) => state.name);

  async function greet() {
    setGreetMsg(await invoke("greet", { name }));
  }

  return (
    <>
      <h1>Welcome MPG Cafe</h1>
      <h2>{serverStatus}</h2>

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
    </>
  );
};
