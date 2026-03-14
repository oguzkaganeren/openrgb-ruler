import { DeviceTarget, Rule, Trigger, RgbAction } from "../types";

interface Props {
  rules: Rule[];
  onToggle: (id: string) => void;
  onEdit: (rule: Rule) => void;
  onDelete: (id: string) => void;
  onAdd: () => void;
}

function describeTrigger(t: Trigger): string {
  switch (t.type) {
    case "SystemLock":    return "Screen locks";
    case "SystemUnlock":  return "Screen unlocks";
    case "ProcessStart":  return `${t.process_name} starts`;
    case "ProcessStop":   return `${t.process_name} stops`;
    case "SessionIdle":   return `Idle for ${t.seconds}s`;
    case "SessionActive": return "Session becomes active";
    case "Suspend":       return "System suspends";
    case "Resume":        return "System resumes";
    case "TimeOfDay": {
      const DAY_NAMES = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
      const dayLabel = t.days.length === 0 ? "every day" : t.days.map(d => DAY_NAMES[d]).join(", ");
      return `At ${t.time} (${dayLabel})`;
    }
  }
}

function describeDeviceTarget(t: DeviceTarget): string {
  if (t.type === "All") return "";
  if (t.ids.length === 1) return ` [device ${t.ids[0]}]`;
  return ` [devices ${t.ids.join(", ")}]`;
}

function describeAction(a: RgbAction): string {
  switch (a.type) {
    case "TurnOff":       return "Turn off LEDs";
    case "SetColor":      return `Set color #${a.hex}`;
    case "LoadProfile":   return `Load profile "${a.name}"`;
    case "SetBrightness": return `Set brightness ${a.percent}%`;
  }
}

function ActionColorDot({ action }: { action: RgbAction }) {
  if (action.type !== "SetColor") return null;
  return (
    <span
      className="color-dot"
      style={{ background: `#${action.hex}` }}
    />
  );
}

export function RuleList({ rules, onToggle, onEdit, onDelete, onAdd }: Props) {
  return (
    <div className="rule-list">
      <div className="rule-list-header">
        <h2 className="rule-list-title">Rules</h2>
        <button className="btn btn-primary" onClick={onAdd}>+ Add Rule</button>
      </div>

      {rules.length === 0 ? (
        <div className="rule-list-empty">
          No rules yet. Click <strong>+ Add Rule</strong> to get started.
        </div>
      ) : (
        <ul className="rule-items">
          {rules.map(rule => (
            <li key={rule.id} className={`rule-item ${rule.enabled ? "" : "rule-disabled"}`}>
              <label className="toggle" title={rule.enabled ? "Disable" : "Enable"}>
                <input
                  type="checkbox"
                  checked={rule.enabled}
                  onChange={() => onToggle(rule.id)}
                />
                <span className="toggle-slider" />
              </label>

              <div className="rule-info">
                <span className="rule-name">{rule.name}</span>
                <span className="rule-desc">
                  {describeTrigger(rule.trigger)}
                  <span className="rule-arrow">→</span>
                  <ActionColorDot action={rule.action} />
                  {describeAction(rule.action)}
                  {describeDeviceTarget(rule.device_target)}
                </span>
              </div>

              <div className="rule-actions">
                <button className="btn-icon" onClick={() => onEdit(rule)} title="Edit">✎</button>
                <button className="btn-icon btn-icon-danger" onClick={() => onDelete(rule.id)} title="Delete">✕</button>
              </div>
            </li>
          ))}
        </ul>
      )}
    </div>
  );
}
