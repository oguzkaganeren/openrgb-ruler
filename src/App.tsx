import { useState } from "react";
import "./App.css";
import { Rule } from "./types";
import { useRules } from "./hooks/useRules";
import { useOpenRgb } from "./hooks/useOpenRgb";
import { useAutostart } from "./hooks/useAutostart";
import { RuleList } from "./components/RuleList";
import { RuleEditor } from "./components/RuleEditor";
import { StatusBar } from "./components/StatusBar";

type View = { mode: "list" } | { mode: "edit"; rule?: Rule };

function App() {
  const { rules, loading, error, addRule, deleteRule, toggleRule, saveRules } = useRules();
  const { available, profiles, devices } = useOpenRgb();
  const { enabled: autostartEnabled, error: autostartError, toggle: toggleAutostart } = useAutostart();
  const [view, setView] = useState<View>({ mode: "list" });

  const handleSave = async (rule: Rule) => {
    if (view.mode === "edit" && view.rule) {
      // Update existing rule in-place
      const updated = rules.map(r => r.id === rule.id ? rule : r);
      await saveRules(updated);
    } else {
      await addRule(rule);
    }
    setView({ mode: "list" });
  };

  return (
    <div className="app">
      <header className="app-header">
        <div className="app-header-inner">
          <span className="app-logo">⬡</span>
          <span className="app-title">OpenRGB Action</span>
        </div>
        <div className="app-header-right">
          <label className="autostart-toggle" title={autostartError ?? "Start automatically on login"}>
            <input type="checkbox" checked={autostartEnabled} onChange={toggleAutostart} />
            <span className="toggle-slider" />
            <span className="autostart-label">Start on login</span>
          </label>
          <StatusBar available={available} />
        </div>
      </header>

      <main className="app-main">
        {loading && <div className="loading">Loading rules…</div>}
        {error && <div className="global-error">Failed to load rules: {error}</div>}

        {!loading && view.mode === "list" && (
          <RuleList
            rules={rules}
            onToggle={toggleRule}
            onEdit={rule => setView({ mode: "edit", rule })}
            onDelete={deleteRule}
            onAdd={() => setView({ mode: "edit" })}
          />
        )}

        {view.mode === "edit" && (
          <RuleEditor
            rule={view.rule}
            profiles={profiles}
            devices={devices}
            onSave={handleSave}
            onCancel={() => setView({ mode: "list" })}
          />
        )}
      </main>
    </div>
  );
}

export default App;
