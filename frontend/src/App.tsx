import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { Menu } from "@tauri-apps/api/menu";
import { getCurrentWindow, Window } from "@tauri-apps/api/window";
import { moveWindow, Position } from "@tauri-apps/plugin-positioner";
import { useEffect } from "react";
import { Toaster } from "react-hot-toast";
import { BrowserRouter } from "react-router-dom";
import Body from "./Body";
import { DarkModeProvider } from "./components/DarkModeProvider";
import "./i18next";
import { tray } from "./lib/tray-icon-helper";

const queryClient = new QueryClient();
const App = () => {
  useEffect(() => {
    const setup = async () => {
      const initTrayIcon = async () => {
        tray.setIcon("icons/tray-icon.png");
        tray.setTitle("Miti Calendar");
        tray.setTooltip("Miti Calendar");
        tray.setVisible(true);
        tray.setShowMenuOnLeftClick(false);
        const menu = await Menu.new({
          items: [
            {
              id: "quit",
              text: "Quit",
            },
            {
              id: "calendar",
              text: "Open Calendar",
              action: () => {
                const currentWindow = Window.getCurrent();
                currentWindow.show();
                currentWindow.setAlwaysOnTop(true);
                moveWindow(Position.BottomRight);
              },
            },
            {
              id: "date-converter",
              text: "AD/BS Date Converter",
            },
            {
              id: "settings",
              text: "Settings",
            },
          ],
        });
        tray.setMenu(menu);

        // hide window after init
        Window.getCurrent().hide();
        Window.getCurrent().setOverlayIcon("");
      };
      await initTrayIcon();
      const unlisten = await getCurrentWindow().onFocusChanged(({ payload: focused }) => {
        if (!focused) {
          Window.getCurrent().hide();
        }
      });
      return unlisten;
    };

    let cleanup: (() => void) | undefined;
    setup().then((unlisten) => {
      cleanup = unlisten;
    });

    return () => cleanup?.();
  }, []);

  return (
    <BrowserRouter>
      <QueryClientProvider client={queryClient}>
        <DarkModeProvider>
          <Body />
          <Toaster position="bottom-center" />
        </DarkModeProvider>
      </QueryClientProvider>
    </BrowserRouter>
  );
};

export default App;
