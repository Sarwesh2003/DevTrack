const navItems = [
  { label: 'Dashboard', href: '#dashboard' },
  { label: 'Today', href: '#today' },
  { label: 'Roadmap', href: '#roadmap' },
  { label: 'DSA', href: '#dsa' },
  { label: 'Core Java', href: '#java' },
  { label: 'LLD', href: '#lld' },
  { label: 'HLD', href: '#hld' },
  { label: 'FDE', href: '#fde' },
  { label: 'Projects', href: '#projects' },
  { label: 'Revision', href: '#revision' },
  { label: 'Analytics', href: '#analytics' },
  { label: 'Settings', href: '#settings' },
]

interface SidebarProps {
  open: boolean
  onToggle: () => void
}

export default function Sidebar({ open, onToggle }: SidebarProps) {
  return (
    <aside
      className={`fixed left-0 top-0 bottom-0 w-16 lg:relative lg:static lg:w-64 border-r border-border bg-background transition-all duration-300 ${!open && 'lg:hidden'}`}
      aria-label="Navigation"
    >
      <div className="flex items-center justify-between h-12 px-4 border-b border-border">
        <span className="font-bold text-sm tracking-tight">DT</span>
        <button
          onClick={onToggle}
          className="rounded-md p-1.5 hover:bg-muted transition-colors"
          aria-label="Toggle sidebar"
        >
          {open ? (
            <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M20 35H4m0 0l15-15M4 20l15-15" /></svg>
          ) : (
            <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 6h16M4 12h16M4 18h16" /></svg>
          )}
        </button>
      </div>
      <nav className="p-3 space-y-1">
        {navItems.map((item) => (
          <a
            key={item.label}
            href={item.href}
            className="flex items-center rounded-md px-3 py-2 text-sm text-muted-foreground hover:bg-muted hover:text-foreground transition-colors"
          >
            {item.label}
          </a>
        ))}
      </nav>
    </aside>
  )
}
