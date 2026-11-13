import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'

// Mock window.__TAURI__.invoke
const mockInvoke = vi.fn()
Object.defineProperty(window, '__TAURI__', {
  value: { invoke: mockInvoke },
  configurable: true,
})

import { api } from './api'

describe('api service', () => {
  beforeEach(() => {
    mockInvoke.mockReset()
  })

  afterEach(() => {
    vi.restoreAllMocks()
  })

  it('healthCheck calls health_check command', async () => {
    mockInvoke.mockResolvedValue({ ok: true, health: { connected: true, path: '/tmp/test.sqlite', version: 1 } })
    const result = await api.healthCheck()
    expect(mockInvoke).toHaveBeenCalledWith('health_check', undefined)
    expect(result.ok).toBe(true)
    expect(result.health.connected).toBe(true)
  })

  it('getSettings calls get_settings command', async () => {
    mockInvoke.mockResolvedValue({ ok: true, settings: { theme: 'dark' } })
    const result = await api.getSettings()
    expect(mockInvoke).toHaveBeenCalledWith('get_settings', undefined)
    expect(result.ok).toBe(true)
  })

  it('saveSettings calls save_settings command', async () => {
    mockInvoke.mockResolvedValue({ ok: true })
    const result = await api.saveSettings({ theme: 'light' })
    expect(mockInvoke).toHaveBeenCalledWith('save_settings', { settings: { theme: 'light' } })
    expect(result.ok).toBe(true)
  })

  it('backupDatabase calls backup_database command', async () => {
    mockInvoke.mockResolvedValue({ ok: true, path: '/backup/devtrack.sqlite', error: undefined })
    const result = await api.backupDatabase('/backup/devtrack.sqlite')
    expect(mockInvoke).toHaveBeenCalledWith('backup_database', { destination: '/backup/devtrack.sqlite' })
    expect(result.ok).toBe(true)
  })

  it('restoreDatabase calls restore_database command', async () => {
    mockInvoke.mockResolvedValue({ ok: true, error: undefined })
    const result = await api.restoreDatabase('/backup/devtrack.sqlite')
    expect(mockInvoke).toHaveBeenCalledWith('restore_database', { source: '/backup/devtrack.sqlite' })
    expect(result.ok).toBe(true)
  })

  it('runMigrations calls run_migrations command', async () => {
    mockInvoke.mockResolvedValue({ ok: true, version: 1, error: undefined })
    const result = await api.runMigrations()
    expect(mockInvoke).toHaveBeenCalledWith('run_migrations', undefined)
    expect(result.ok).toBe(true)
  })
})
