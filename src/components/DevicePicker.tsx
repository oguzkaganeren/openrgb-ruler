import { DeviceTarget, RgbDevice } from "../types";

interface Props {
  value: DeviceTarget;
  devices: RgbDevice[];
  onChange: (target: DeviceTarget) => void;
}

export function DevicePicker({ value, devices, onChange }: Props) {
  const isAll = value.type === "All";
  const selectedIds = value.type === "Specific" ? value.ids : [];

  const handleAllChange = (checked: boolean) => {
    if (checked) {
      onChange({ type: "All" });
    } else {
      // Default to all individual devices selected when unchecking "All"
      onChange({ type: "Specific", ids: devices.map(d => d.id) });
    }
  };

  const handleDeviceToggle = (id: number, checked: boolean) => {
    const next = checked
      ? [...selectedIds, id]
      : selectedIds.filter(i => i !== id);
    if (next.length === 0) {
      onChange({ type: "All" });
    } else {
      onChange({ type: "Specific", ids: next });
    }
  };

  if (devices.length === 0) {
    return (
      <div className="device-picker">
        <label className="device-all">
          <input type="checkbox" checked disabled />
          <span>All devices</span>
        </label>
        <p className="device-picker-hint">No devices detected — all devices will be targeted.</p>
      </div>
    );
  }

  return (
    <div className="device-picker">
      <label className="device-all">
        <input
          type="checkbox"
          checked={isAll}
          onChange={e => handleAllChange(e.target.checked)}
        />
        <span>All devices</span>
      </label>

      {!isAll && (
        <ul className="device-list">
          {devices.map(d => (
            <li key={d.id} className="device-item">
              <label>
                <input
                  type="checkbox"
                  checked={selectedIds.includes(d.id)}
                  onChange={e => handleDeviceToggle(d.id, e.target.checked)}
                />
                <span className="device-index">{d.id}</span>
                <span className="device-name">{d.name}</span>
              </label>
            </li>
          ))}
        </ul>
      )}
    </div>
  );
}
