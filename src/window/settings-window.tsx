import { invoke } from "@tauri-apps/api/core";
import { useEffect } from "react";

interface UIConfig {
  cafe_name: string;
  station_id: string;
  insert_coin_text: string;
  autoshutdown_text: string;
  smwindow_position: string;
  background_img: string;
  countdown_timer: number;
}

export const SettingsWindow = () => {
  useEffect(() => {
    const handleFetchConfig = async () => {
      const response = await invoke<UIConfig>("get_ui_config");
      console.log("Received UI config:", response);
    };
    handleFetchConfig();
  }, []);

  return <div>Showing Settings Main window</div>;
};

