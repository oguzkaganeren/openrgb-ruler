import { Trigger } from "../types";

interface Props {
  value: Trigger;
  onChange: (t: Trigger) => void;
}

const TRIGGER_TYPES = [
  { value: "SystemLock",    label: "System Lock" },
  { value: "SystemUnlock",  label: "System Unlock" },
  { value: "ProcessStart",  label: "Process Start" },
  { value: "ProcessStop",   label: "Process Stop" },
  { value: "SessionIdle",   label: "Session Idle" },
  { value: "SessionActive", label: "Session Active (idle ends)" },
  { value: "Suspend",       label: "Suspend (going to sleep)" },
  { value: "Resume",        label: "Resume (waking from sleep)" },
  { value: "TimeOfDay",     label: "Time of Day" },
];

function defaultTrigger(type: string): Trigger {
  switch (type) {
    case "ProcessStart":  return { type: "ProcessStart",  process_name: "" };
    case "ProcessStop":   return { type: "ProcessStop",   process_name: "" };
    case "SessionIdle":   return { type: "SessionIdle",   seconds: 300 };
    case "SessionActive": return { type: "SessionActive" };
    case "Suspend":       return { type: "Suspend" };
    case "Resume":        return { type: "Resume" };
    case "TimeOfDay":     return { type: "TimeOfDay",     time: "09:00", days: [] };
    case "SystemUnlock":  return { type: "SystemUnlock" };
    default:              return { type: "SystemLock" };
  }
}

export function TriggerSelector({ value, onChange }: Props) {
  const handleTypeChange = (type: string) => {
    onChange(defaultTrigger(type));
  };

  return (
    <div className="selector">
      <label className="field-label">Trigger type</label>
      <select
        className="select"
        value={value.type}
        onChange={e => handleTypeChange(e.target.value)}
      >
        {TRIGGER_TYPES.map(t => (
          <option key={t.value} value={t.value}>{t.label}</option>
        ))}
      </select>

      {(value.type === "ProcessStart" || value.type === "ProcessStop") && (
        <div className="field-group">
          <label className="field-label">Process name</label>
          <input
            className="input"
            type="text"
            placeholder="e.g. firefox, steam, mpv"
            value={value.process_name}
            onChange={e => onChange({ ...value, process_name: e.target.value })}
          />
        </div>
      )}

      {value.type === "SessionIdle" && (
        <div className="field-group">
          <label className="field-label">Idle seconds</label>
          <input
            className="input"
            type="number"
            min={1}
            value={value.seconds}
            onChange={e => onChange({ ...value, seconds: Number(e.target.value) })}
          />
        </div>
      )}

      {value.type === "TimeOfDay" && (
        <div className="field-group">
          <label className="field-label">Time</label>
          <input
            className="input"
            type="time"
            value={value.time}
            onChange={e => onChange({ ...value, time: e.target.value })}
          />
          <label className="field-label" style={{ marginTop: "0.5rem" }}>Days (leave empty for every day)</label>
          <div className="day-picker">
            {["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"].map((day, i) => (
              <label key={i} className="day-checkbox">
                <input
                  type="checkbox"
                  checked={value.days.includes(i)}
                  onChange={e => {
                    const days = e.target.checked
                      ? [...value.days, i].sort()
                      : value.days.filter(d => d !== i);
                    onChange({ ...value, days });
                  }}
                />
                {day}
              </label>
            ))}
          </div>
        </div>
      )}
    </div>
  );
}
