export interface DbHealth {
  connected: boolean
  path: string
  version: number
}

export interface Settings {
  data_root?: string
  weekly_capacity_hours?: number
  timezone?: string
  theme?: string
  dsa_cursor_category?: string
  lld_cursor_topic?: string
}

export interface HealthResponse {
  ok: boolean
  health: DbHealth
}

export interface SettingsResponse {
  ok: boolean
  settings: Settings
}

export interface SaveSettingsResponse {
  ok: boolean
}

export interface BackupResponse {
  ok: boolean
  path: string
  error?: string
}

export interface RestoreResponse {
  ok: boolean
  error?: string
}

export interface MigrationsResponse {
  ok: boolean
  version: number
  error?: string
}

export interface BackupRequest {
  destination: string
}

export interface RestoreRequest {
  source: string
}
