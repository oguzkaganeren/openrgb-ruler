interface Props {
  value: string;
  onChange: (hex: string) => void;
}

export function ColorPicker({ value, onChange }: Props) {
  const normalized = value.startsWith("#") ? value : `#${value}`;

  return (
    <div className="color-picker">
      <input
        type="color"
        value={normalized}
        onChange={e => onChange(e.target.value.replace("#", ""))}
        className="color-swatch"
      />
      <input
        type="text"
        value={value}
        maxLength={6}
        placeholder="RRGGBB"
        onChange={e => {
          const v = e.target.value.replace(/[^0-9a-fA-F]/g, "").slice(0, 6);
          onChange(v);
        }}
        className="color-hex-input"
      />
    </div>
  );
}
