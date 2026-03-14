interface Props {
  available: boolean | null;
}

export function StatusBar({ available }: Props) {
  const label =
    available === null ? "Checking OpenRGB..." :
    available ? "OpenRGB: available" :
    "OpenRGB: not found in PATH";

  const cls =
    available === null ? "status-checking" :
    available ? "status-ok" :
    "status-error";

  return (
    <div className={`status-bar ${cls}`}>
      <span className="status-dot" />
      {label}
    </div>
  );
}
