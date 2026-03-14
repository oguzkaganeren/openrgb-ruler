import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { DeviceTarget, Rule, RgbDevice, Trigger, RgbAction } from "../types";
import { TriggerSelector } from "./TriggerSelector";
import { ActionSelector } from "./ActionSelector";
import { DevicePicker } from "./DevicePicker";

interface Props {
  rule?: Rule;
  profiles: string[];
  devices: RgbDevice[];
  onSave: (rule: Rule) => Promise<void>;
  onCancel: () => void;
}

const DEFAULT_TRIGGER: Trigger = { type: "SystemLock" };
const DEFAULT_ACTION: RgbAction = { type: "TurnOff" };
const DEFAULT_DEVICE_TARGET: DeviceTarget = { type: "All" };

export function RuleEditor({ rule, profiles, devices, onSave, onCancel }: Props) {
  const [name, setName] = useState(rule?.name ?? "");
  const [trigger, setTrigger] = useState<Trigger>(rule?.trigger ?? DEFAULT_TRIGGER);
  const [action, setAction] = useState<RgbAction>(rule?.action ?? DEFAULT_ACTION);
  const [deviceTarget, setDeviceTarget] = useState<DeviceTarget>(rule?.device_target ?? DEFAULT_DEVICE_TARGET);
  const [testing, setTesting] = useState(false);
  const [testResult, setTestResult] = useState<string | null>(null);
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleSave = async () => {
    if (!name.trim()) { setError("Rule name is required"); return; }
    setSaving(true);
    setError(null);
    try {
      const id = rule?.id ?? await invoke<string>("generate_id");
      await onSave({
        id,
        name: name.trim(),
        enabled: rule?.enabled ?? true,
        trigger,
        action,
        device_target: deviceTarget,
      });
    } catch (e) {
      setError(String(e));
    } finally {
      setSaving(false);
    }
  };

  const handleTest = async () => {
    setTesting(true);
    setTestResult(null);
    try {
      await invoke("test_action", { action, deviceTarget });
      setTestResult("Action executed successfully");
    } catch (e) {
      setTestResult(`Error: ${e}`);
    } finally {
      setTesting(false);
    }
  };

  return (
    <div className="rule-editor">
      <h2 className="editor-title">{rule ? "Edit Rule" : "New Rule"}</h2>

      <div className="field-group">
        <label className="field-label">Rule name</label>
        <input
          className="input"
          type="text"
          placeholder="e.g. Lock screen → LEDs off"
          value={name}
          onChange={e => setName(e.target.value)}
        />
      </div>

      <div className="editor-columns">
        <div className="editor-col">
          <h3 className="col-title">Trigger</h3>
          <TriggerSelector value={trigger} onChange={setTrigger} />
        </div>
        <div className="editor-divider" />
        <div className="editor-col">
          <h3 className="col-title">Action</h3>
          <ActionSelector value={action} profiles={profiles} onChange={setAction} />
        </div>
      </div>

      <div className="field-group">
        <h3 className="col-title">Devices</h3>
        <DevicePicker value={deviceTarget} devices={devices} onChange={setDeviceTarget} />
      </div>

      {testResult && (
        <div className={`test-result ${testResult.startsWith("Error") ? "test-error" : "test-ok"}`}>
          {testResult}
        </div>
      )}

      {error && <div className="form-error">{error}</div>}

      <div className="editor-actions">
        <button className="btn btn-secondary" onClick={handleTest} disabled={testing}>
          {testing ? "Testing…" : "Test Action"}
        </button>
        <div className="editor-actions-right">
          <button className="btn btn-ghost" onClick={onCancel}>Cancel</button>
          <button className="btn btn-primary" onClick={handleSave} disabled={saving}>
            {saving ? "Saving…" : "Save Rule"}
          </button>
        </div>
      </div>
    </div>
  );
}
