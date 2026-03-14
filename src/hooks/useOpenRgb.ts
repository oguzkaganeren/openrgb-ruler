import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { RgbDevice } from "../types";

export function useOpenRgb() {
  const [available, setAvailable] = useState<boolean | null>(null);
  const [profiles, setProfiles] = useState<string[]>([]);
  const [devices, setDevices] = useState<RgbDevice[]>([]);

  useEffect(() => {
    invoke<boolean>("check_openrgb_available").then(setAvailable);
    invoke<string[]>("get_openrgb_profiles")
      .then(setProfiles)
      .catch(() => setProfiles([]));
    invoke<RgbDevice[]>("get_openrgb_devices")
      .then(setDevices)
      .catch(() => setDevices([]));
  }, []);

  return { available, profiles, devices };
}
