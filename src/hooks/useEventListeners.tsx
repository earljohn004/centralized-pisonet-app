import { listen, Event } from "@tauri-apps/api/event";
import { useEffect } from "react";
import { router } from "../Router";
import { LicenseInformation, useSettingStore } from "../store/Settings";

export const useEventListeners = () => {
  const setServerStatus = useSettingStore((state) => state.setServerStatus);
  const setCoin = useSettingStore((state) => state.setCoin);
  const setTimerDone = useSettingStore((state) => state.setTimerDone);
  const setRemainingTime = useSettingStore((state) => state.setRemainingTime);
  const setLicenseInformation = useSettingStore(
    (state) => state.setLicenseInformation,
  );

  useEffect(() => {
    const unlistenLicenseInformation = listen("handler_initialize_license", (event) => {
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
    const unlistenNavigate = listen(
      "handler_settings_route",
      (event: Event<boolean>) => {
        console.log("Received settings route handler", event.payload);
        if (event.payload) {
          router.navigate("/settings");
        }
      },
    );
    return () => {
      unlistenNavigate.then((unlistenFn) => unlistenFn());
    };
  }, []);

  useEffect(() => {
    const unlistenRegister = listen("register_request", (event) => {
      console.log("Received a register request", event.payload);
      setServerStatus("Received a valid request");
    });

    const unlistenAddTime = listen(
      "handler_addtime",
      (event: Event<number>) => {
        console.log("Received add time request", event.payload);
        setCoin(event.payload);
        setTimerDone(false);
        router.navigate("/show_small");
      },
    );

    const unlistenTimerUpdate = listen(
      "handler_timer_update",
      (event: Event<number>) => {
        setRemainingTime(event.payload);
      },
    );

    const unlistenTimerDone = listen("handler_timer_done", () => {
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
