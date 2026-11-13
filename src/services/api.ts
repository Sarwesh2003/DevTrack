import {
  HealthResponse,
  SettingsResponse,
  SaveSettingsResponse,
  BackupResponse,
  RestoreResponse,
  MigrationsResponse,
  Settings,
} from '@/types'

// Tauri command invocations are available via window.__TAURI__
// In production, these come from @tauri-apps/api
declare global {
  interface Window {
    __TAURI__?: {
      invoke: (cmd: string, args?: Record<string, unknown>) => Promise<unknown>
    }
  }
}

async function invoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  if (typeof window !== 'undefined' && window.__TAURI__?.invoke) {
    return window.__TAURI__.invoke(cmd, args) as Promise<T>
  }
  // Dev fallback: return sensible defaults so the UI works without Tauri
  switch (cmd) {
    case 'health_check':
      return { ok: true, health: { connected: false, path: '', version: 0 } } as T
    case 'get_settings':
      return { ok: true, settings: {} as Settings } as T
    case 'save_settings':
      return { ok: true } as T
    case 'backup_database':
      return { ok: true, path: '', error: undefined } as T
    case 'restore_database':
      return { ok: true, error: undefined } as T
    case 'run_migrations':
      return { ok: true, version: 1, error: undefined } as T
    default:
      throw new Error(`Unknown command: ${cmd}`)
  }
}

export const api = {
  healthCheck: () => invoke<HealthResponse>('health_check'),
  getSettings: () => invoke<SettingsResponse>('get_settings'),
  saveSettings: (settings: Settings) =>
    invoke<SaveSettingsResponse>('save_settings', { settings }),
  backupDatabase: (destination: string) =>
    invoke<BackupResponse>('backup_database', { destination }),
  restoreDatabase: (source: string) =>
    invoke<RestoreResponse>('restore_database', { source }),
  runMigrations: () => invoke<MigrationsResponse>('run_migrations'),
}
