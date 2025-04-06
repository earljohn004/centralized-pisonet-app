import { create } from "zustand";

export interface LicenseInformation {
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

