import { useState, useEffect, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Rule } from "../types";

export function useRules() {
  const [rules, setRules] = useState<Rule[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const load = useCallback(async () => {
    try {
      const data = await invoke<Rule[]>("get_rules");
      setRules(data);
    } catch (e) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => { load(); }, [load]);

  const addRule = useCallback(async (rule: Rule) => {
    await invoke("add_rule", { rule });
    await load();
  }, [load]);

  const deleteRule = useCallback(async (id: string) => {
    await invoke("delete_rule", { id });
    await load();
  }, [load]);

  const toggleRule = useCallback(async (id: string) => {
    await invoke("toggle_rule", { id });
    setRules(prev => prev.map(r => r.id === id ? { ...r, enabled: !r.enabled } : r));
  }, []);

  const saveRules = useCallback(async (updated: Rule[]) => {
    await invoke("save_rules", { rules: updated });
    setRules(updated);
  }, []);

  const generateId = useCallback(async (): Promise<string> => {
    return invoke<string>("generate_id");
  }, []);

  return { rules, loading, error, addRule, deleteRule, toggleRule, saveRules, generateId, reload: load };
}
