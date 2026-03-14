interface Props {
  profiles: string[];
  value: string;
  onChange: (name: string) => void;
}

export function ProfilePicker({ profiles, value, onChange }: Props) {
  return (
    <div className="profile-picker">
      {profiles.length > 0 ? (
        <select value={value} onChange={e => onChange(e.target.value)} className="select">
          <option value="">— select profile —</option>
          {profiles.map(p => <option key={p} value={p}>{p}</option>)}
        </select>
      ) : (
        <input
          type="text"
          value={value}
          placeholder="Profile name"
          onChange={e => onChange(e.target.value)}
          className="input"
        />
      )}
      {profiles.length === 0 && (
        <span className="hint">No profiles found — enter name manually</span>
      )}
    </div>
  );
}
