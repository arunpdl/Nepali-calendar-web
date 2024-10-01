import { Switch } from "@headlessui/react";
import { useEffect, useState } from "react";
import { disable, enable, isEnabled } from "tauri-plugin-autostart-api";

const AppSettings = () => {
  const [autoStartEnabled, setAutoStartEnabled] = useState(false);

  useEffect(() => {
    isEnabled().then((enabled) => {
      setAutoStartEnabled(enabled);
    });
  }, []);

  const handleChangeAutoStart = async () => {
    if (autoStartEnabled) {
      // Disable autostart
      await disable();
    } else {
      // Enable autostart
      await enable();
    }
    setAutoStartEnabled(!autoStartEnabled);
  };

  return (
    <div className="p-8">
      <div className="mb-10 text-center text-2xl font-bold dark:text-white">Application Settings</div>
      <div className="flex items-center gap-4">
        <label htmlFor="autoStart" className="text-xl dark:text-white">
          Enable Autostart at login
        </label>
        <Switch
          id="autoStart"
          checked={autoStartEnabled}
          onChange={handleChangeAutoStart}
          className={`${
            autoStartEnabled ? "bg-blue-600" : "bg-gray-200"
          } relative inline-flex h-6 w-11 items-center rounded-full`}>
          <span className="sr-only">Enable autostart at login</span>
          <span
            className={`${
              autoStartEnabled ? "translate-x-6" : "translate-x-1"
            } inline-block h-4 w-4 transform rounded-full bg-white transition`}
          />
        </Switch>
      </div>
    </div>
  );
};

export default AppSettings;
