import { listen, Event } from "@tauri-apps/api/event";
import { useEffect } from "react";
import { create } from "zustand";
import { router } from "./Router";

type SettingState = {
  greetMsg: string;
  setGreetMsg: (msg: string) => void;
  name: string;
  setName: (name: string) => void;
  serverStatus: string;
  setServerStatus: (status: string) => void;
  coin: number;
  setCoin: (coin: number) => void;
  remainingTime: number;
  setRemainingTime: (time: number) => void;
  timerDone: boolean;
  setTimerDone: (done: boolean) => void;
};

export const useSettingStore = create<SettingState>((set) => ({
  greetMsg: "",
  name: "",
  serverStatus: "offline",
  coin: 0,
  remainingTime: 0,
  timerDone: false,
  setGreetMsg: (msg) => set({ greetMsg: msg }),
  setName: (name) => set({ name }),
  setServerStatus: (status) => set({ serverStatus: status }),
  setCoin: (coin) => set({ coin }),
  setRemainingTime: (time) => set({ remainingTime: time }),
  setTimerDone: (done) => set({ timerDone: done }),
}));

export const useEventListeners = () => {
  const setServerStatus = useSettingStore((state) => state.setServerStatus);
  const setCoin = useSettingStore((state) => state.setCoin);
  const setTimerDone = useSettingStore((state) => state.setTimerDone);
  const setRemainingTime = useSettingStore((state) => state.setRemainingTime);

  useEffect(() => {
    const unlistenRegister = listen("register_request", (event) => {
      console.log("Received a received request", event.payload);
      setServerStatus("Received a valid request");
    });

    const unlistenAddTime = listen(
      "addtime_handler",
      (event: Event<number>) => {
        console.log("Received add time request", event.payload);
        setCoin(event.payload);
        setTimerDone(false);
        router.navigate("/show_small");
      },
    );

    const unlistenTimerUpdate = listen(
      "timer_update",
      (event: Event<number>) => {
        setRemainingTime(event.payload);
      },
    );

    const unlistenTimerDone = listen("timer_done", () => {
      setTimerDone(true);
      router.navigate("/show_main");
    });

    return () => {
      unlistenRegister.then((unlistenFn) => unlistenFn());
      unlistenAddTime.then((unlistenFn) => unlistenFn());
      unlistenTimerUpdate.then((unlistenFn) => unlistenFn());
      unlistenTimerDone.then((unlistenFn) => unlistenFn());
    };
  }, []);

  useEffect(() => {
    const handleKeyDown = (event: KeyboardEvent) => {
      if (
        event.key === "F5" ||
        (event.ctrlKey && event.key === "r") ||
        (event.metaKey && event.key === "r")
      ) {
        event.preventDefault();
        console.log("Refresh is disabled");
      }
    };

    const handleContextMenu = (event: MouseEvent) => {
      if (import.meta.env.PROD) {
        event.preventDefault();
      }
    };

    window.addEventListener("keydown", handleKeyDown);
    window.addEventListener("contextmenu", handleContextMenu);

    return () => {
      window.removeEventListener("keydown", handleKeyDown);
      window.removeEventListener("contextmenu", handleContextMenu);
    };
  }, []);
};
