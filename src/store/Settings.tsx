import { listen, Event } from "@tauri-apps/api/event";
import { useEffect } from "react";
import { create } from "zustand";
import { router } from "../Router";

interface LicenseInformation {
  authorized: boolean;
  serialNumber: string;
  emailAddress: string;
}

type SettingState = {
  serverStatus: string;
  setServerStatus: (status: string) => void;
  coin: number;
  setCoin: (coin: number) => void;
  remainingTime: number;
  setRemainingTime: (time: number) => void;
  timerDone: boolean;
  setTimerDone: (done: boolean) => void;
  licenseInformation: LicenseInformation;
  setLicenseInformation: (info: LicenseInformation) => void;
};

export const useSettingStore = create<SettingState>((set) => ({
  serverStatus: "offline",
  coin: 0,
  remainingTime: 0,
  timerDone: false,
  licenseInformation: {
    authorized: false,
    serialNumber: "",
    emailAddress: "",
  },
  setLicenseInformation: (info: LicenseInformation) =>
    set({
      licenseInformation: {
        authorized: info.authorized,
        emailAddress: info.emailAddress,
        serialNumber: info.serialNumber,
      },
    }),
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
  const setLicenseInformation = useSettingStore(
    (state) => state.setLicenseInformation,
  );

  useEffect(() => {
    const unlistenLicenseInformation = listen("initialize_license", (event) => {
      const { authorized, serialNumber, emailAddress } =
        event.payload as LicenseInformation;
      console.log("Received license information", event.payload);
      setLicenseInformation({ authorized, serialNumber, emailAddress });
    });

    return () => {
      unlistenLicenseInformation.then((unlistenFn) => unlistenFn());
    };
  }, []);

  useEffect(() => {
    const unlistenRegister = listen("register_request", (event) => {
      console.log("Received a register request", event.payload);
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
