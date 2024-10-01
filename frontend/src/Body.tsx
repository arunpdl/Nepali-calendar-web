import { listen } from "@tauri-apps/api/event";
import { useContext, useEffect } from "react";
import { Route, Routes, useNavigate } from "react-router-dom";
import AppSettings from "./components/AppSettings.tsx";
import { DarkModeContext } from "./components/DarkModeProvider.tsx";
import About from "./pages/About.tsx";
import DateConverter from "./pages/DateConverter.tsx";
import GoogleApiDisclosure from "./pages/GoogleApiDisclosure.tsx";
import Home from "./pages/Home.tsx";
import PrivacyPolicy from "./pages/PrivacyPolicy.tsx";

declare global {
  interface Window {
    __TAURI__: {
      window: {
        appWindow: {
          hide: () => void;
        };
      };
    };
  }
}

const Body = () => {
  const { darkMode } = useContext(DarkModeContext);
  const navigate = useNavigate();

  useEffect(() => {
    const unlisten = listen("navigate", (event) => {
      console.log("navigate", event.payload);
      const payload = event.payload as { route: string; window_id: string };
      navigate("/" + payload.route);
    });

    return () => {
      unlisten.then((f) => f());
    };
  }, [navigate]);

  useEffect(() => {
    //detect click outside the window
    window.addEventListener("click", (e) => {
      const windowElement = document.getElementById("root");
      if (windowElement && !windowElement.contains(e.target as Node)) {
        window.__TAURI__.window.appWindow.hide();
      }
    });

    return () => {
      window.removeEventListener("click", (e) => {
        const windowElement = document.getElementById("root");
        if (windowElement && !windowElement.contains(e.target as Node)) {
          window.__TAURI__.window.appWindow.hide();
        }
      });
    };
  }, []);

  return (
    <div className={(darkMode ? "dark" : "") + " flex min-h-screen flex-col overflow-hidden "}>
      {/* <Navbar /> */}
      <div className="flex-grow">
        <Routes>
          <Route path="/:pageType?/:BSYear?/:BSMonth?" element={<Home />} />
          <Route path="/app-settings" element={<AppSettings />} />
          <Route path="/privacy" element={<PrivacyPolicy />} />
          <Route path="/converter" element={<DateConverter />} />
          <Route path="/about" element={<About />} />
          <Route path="/google-api-disclosure" element={<GoogleApiDisclosure />} />
        </Routes>
      </div>
      {/* <Footer /> */}
    </div>
  );
};

export default Body;
