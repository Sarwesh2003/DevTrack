import { useState, useEffect } from 'react'
import Shell from '@/components/layout/Shell'
import Dashboard from '@/features/dashboard/Dashboard'
import Settings from '@/features/settings/Settings'
import Roadmap from '@/features/roadmap/Roadmap'
import Review from '@/features/review/Review'
import DSA from '@/features/dsa/DSA'
import Workspace from '@/features/workspace/Workspace'
import Projects from '@/features/projects/Projects'
import Analytics from '@/features/analytics/Analytics'
import Search from '@/features/search/Search'

type Page = 'dashboard' | 'settings' | 'roadmap' | 'review' | 'dsa' | 'workspace' | 'projects' | 'analytics' | 'search'
type Theme = 'dark' | 'light' | 'system'

export default function App() {
  const [page, setPage] = useState<Page>('dashboard')
  const [theme, setTheme] = useState<Theme>('dark')

  useEffect(() => {
    const root = window.document.documentElement
    if (theme === 'system') {
      const dark = window.matchMedia('(prefers-color-scheme: dark)').matches
      root.classList.toggle('dark', dark)
    } else {
      root.classList.toggle('dark', theme === 'dark')
    }
  }, [theme])

  return (
    <Shell page={page} onNavigate={setPage}>
      {page === 'dashboard' ? (
        <Dashboard onNavigate={setPage} />
      ) : page === 'settings' ? (
        <Settings onNavigate={setPage} theme={theme} setTheme={setTheme} />
      ) : page === 'review' ? (
        <Review onNavigate={setPage} />
      ) : page === 'dsa' ? (
        <DSA onNavigate={setPage} />
      ) : page === 'workspace' ? (
        <Workspace onNavigate={setPage} />
      ) : page === 'projects' ? (
        <Projects onNavigate={setPage} />
      ) : page === 'analytics' ? (
        <Analytics onNavigate={setPage} />
      ) : page === 'search' ? (
        <Search onNavigate={setPage} />
      ) : (
        <Roadmap onNavigate={setPage} />
      )}
    </Shell>
  )
}
