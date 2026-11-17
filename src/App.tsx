import { useState, useEffect } from 'react'
import Shell from '@/components/layout/Shell'
import Dashboard from '@/features/dashboard/Dashboard'
import Settings from '@/features/settings/Settings'
import Roadmap from '@/features/roadmap/Roadmap'

type Page = 'dashboard' | 'settings' | 'roadmap'
type Theme = 'dark' | 'light' | 'system'

export default function App() {
  const [page, setPage] = useState<Page>('dashboard')
  const [theme, setTheme] = useState<Theme>('dark')

  useEffect(() => {
    const root = window.document.documentElement
    root.setAttribute('data-theme', theme)
  }, [theme])

  return (
    <Shell page={page} onNavigate={setPage}>
      {page === 'dashboard' ? (
        <Dashboard onNavigate={setPage} />
      ) : page === 'settings' ? (
        <Settings onNavigate={setPage} setTheme={setTheme} />
      ) : (
        <Roadmap onNavigate={setPage} />
      )}
    </Shell>
  )
}
