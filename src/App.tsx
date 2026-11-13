import { useState } from 'react'
import Shell from '@/components/layout/Shell'
import Dashboard from '@/features/dashboard/Dashboard'
import Settings from '@/features/settings/Settings'

type Page = 'dashboard' | 'settings'

export default function App() {
  const [page, setPage] = useState<Page>('dashboard')

  const navigate = (next: Page) => setPage(next)

  return (
    <Shell>
      {page === 'dashboard' ? <Dashboard onNavigate={navigate} /> : <Settings onNavigate={navigate} />}
    </Shell>
  )
}
