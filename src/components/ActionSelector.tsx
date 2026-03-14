import { RgbAction } from "../types";
import { ColorPicker } from "./ColorPicker";
import { ProfilePicker } from "./ProfilePicker";

interface Props {
  value: RgbAction;
  profiles: string[];
  onChange: (a: RgbAction) => void;
}

const ACTION_TYPES = [
  { value: "TurnOff",     label: "Turn Off LEDs" },
  { value: "SetColor",    label: "Set Color" },
  { value: "LoadProfile", label: "Load Profile" },
];

function defaultAction(type: string): RgbAction {
  switch (type) {
    case "SetColor":    return { type: "SetColor", hex: "ff0000", percent: 100 };
    case "LoadProfile": return { type: "LoadProfile", name: "" };
    default:            return { type: "TurnOff" };
  }
}

export function ActionSelector({ value, profiles, onChange }: Props) {
  const handleTypeChange = (type: string) => {
    onChange(defaultAction(type));
  };

  return (
    <div className="selector">
      <label className="field-label">Action type</label>
      <select
        className="select"
        value={value.type}
        onChange={e => handleTypeChange(e.target.value)}
      >
        {ACTION_TYPES.map(a => (
          <option key={a.value} value={a.value}>{a.label}</option>
        ))}
      </select>

      {value.type === "SetColor" && (
        <>
          <div className="field-group">
            <label className="field-label">Color</label>
            <ColorPicker
              value={value.hex}
              onChange={hex => onChange({ ...value, hex })}
            />
          </div>
          <div className="field-group">
            <label className="field-label">Brightness: {value.percent}%</label>
            <input
              className="slider"
              type="range"
              min={0}
              max={100}
              value={value.percent}
              onChange={e => onChange({ ...value, percent: Number(e.target.value) })}
            />
          </div>
        </>
      )}

      {value.type === "LoadProfile" && (
        <div className="field-group">
          <label className="field-label">Profile</label>
          <ProfilePicker
            profiles={profiles}
            value={value.name}
            onChange={name => onChange({ ...value, name })}
          />
        </div>
      )}
    </div>
  );
}
