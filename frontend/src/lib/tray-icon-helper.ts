import { TrayIcon, TrayIconOptions } from "@tauri-apps/api/tray";

const options: TrayIconOptions = {
  title: "Miti Calendar",
  tooltip: "Miti Calendar",
  // icon: "./assets/icon-96x96.png",
};

export const tray = await TrayIcon.new(options);
