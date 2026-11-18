import { ReactNode, useState } from 'react'
import Sidebar from '@/components/layout/Sidebar'

type Page = 'dashboard' | 'settings' | 'roadmap' | 'review' | 'dsa'

interface ShellProps {
  page: Page
  onNavigate: (p: Page) => void
  children: ReactNode
}

export default function Shell({ page, onNavigate, children }: ShellProps) {
  const [sidebarOpen, setSidebarOpen] = useState(true)

  return (
    <div className="flex h-screen bg-background text-foreground transition-colors duration-300">
      <Sidebar
        open={sidebarOpen}
        onToggle={() => setSidebarOpen(!sidebarOpen)}
        onNavigate={onNavigate}
        activePage={page}
      />
      <div className="flex-1 flex flex-col overflow-hidden">
        <header className="border-b border-border px-6 py-3 flex items-center justify-between">
          <h1 className="text-xl font-semibold">DevTrack</h1>
          <div className="flex items-center gap-3">
            <button
              className="text-sm px-3 py-1.5 rounded-md hover:bg-muted transition-colors"
              onClick={() => setSidebarOpen(!sidebarOpen)}
            >
              {sidebarOpen ? 'Collapse' : 'Expand'}
            </button>
          </div>
        </header>
        <main className="flex-1 overflow-y-auto p-6">
          {children}
        </main>
      </div>
    </div>
  )
}
