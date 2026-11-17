import { useState, useEffect } from 'react'
import { api } from '@/services/api'
import type { Settings } from '@/types'

export function useSettings() {
  const [settings, setSettings] = useState<Settings>({})
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    api.getSettings()
      .then((res) => {
        if (res.ok) setSettings(res.settings)
        else setError('Failed to load settings')
      })
      .catch(() => setError('Failed to load settings'))
      .finally(() => setLoading(false))
  }, [])

  const save = async (next: Settings): Promise<boolean> => {
    try {
      const res = await api.saveSettings(next)
      if (res.ok) {
        setSettings(next)
        return true
      }
      setError('Failed to save settings')
      return false
    } catch {
      setError('Failed to save settings')
      return false
    }
  }

  return { settings, loading, error, save }
}
