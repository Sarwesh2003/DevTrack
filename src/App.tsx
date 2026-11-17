import { useState } from 'react'
import Shell from '@/components/layout/Shell'
import Dashboard from '@/features/dashboard/Dashboard'
import Settings from '@/features/settings/Settings'
import Roadmap from '@/features/roadmap/Roadmap'

type Page = 'dashboard' | 'settings' | 'roadmap'

export default function App() {
  const [page, setPage] = useState<Page>('dashboard')

  const navigate = (next: Page) => setPage(next)

  return (
    <Shell>
      {page === 'dashboard' ? (
        <Dashboard onNavigate={navigate} />
      ) : page === 'settings' ? (
        <Settings onNavigate={navigate} />
      ) : (
        <Roadmap onNavigate={navigate} />
      )}
    </Shell>
  )
}
