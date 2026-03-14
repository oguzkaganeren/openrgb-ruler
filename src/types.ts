export type Trigger =
  | { type: "SystemLock" }
  | { type: "SystemUnlock" }
  | { type: "ProcessStart"; process_name: string }
  | { type: "ProcessStop"; process_name: string }
  | { type: "SessionIdle"; seconds: number }
  | { type: "SessionActive" }
  | { type: "Suspend" }
  | { type: "Resume" }
  | { type: "TimeOfDay"; time: string; days: number[] };

export type RgbAction =
  | { type: "TurnOff" }
  | { type: "SetColor"; hex: string; percent: number }
  | { type: "LoadProfile"; name: string };

export type DeviceTarget =
  | { type: "All" }
  | { type: "Specific"; ids: number[] };

export interface RgbDevice {
  id: number;
  name: string;
}

export interface Rule {
  id: string;
  name: string;
  enabled: boolean;
  trigger: Trigger;
  action: RgbAction;
  device_target: DeviceTarget;
}
