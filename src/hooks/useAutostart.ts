import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";

export function useAutostart() {
  const [enabled, setEnabled] = useState(false);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    invoke<boolean>("get_autostart")
      .then(v => { setEnabled(v); setLoading(false); })
      .catch(e => { setError(String(e)); setLoading(false); });
  }, []);

  const toggle = async () => {
    const next = !enabled;
    try {
      await invoke("set_autostart", { enabled: next });
      setEnabled(next);
      setError(null);
    } catch (e) {
      setError(String(e));
    }
  };

  return { enabled, loading, error, toggle };
}
