import { useEffect } from 'react'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { X } from 'lucide-react'
import { RepairPanel, type RepairBackend, type RepairProgress } from '../../kryoto-desktop/src/library/RepairPanel'
import { applyLook, type Settings } from '../../kryoto-desktop/src/lib/settings'

const backend: RepairBackend = {
  sources: () => invoke('fixer_sources'),
  state: () => invoke('fixer_state'),
  apply: (source) => invoke('fixer_apply', { source }),
  undo: () => invoke('fixer_undo'),
  support: () => invoke('fixer_support'),
  openSupport: () => invoke('fixer_support_open'),
  listen: (handler) => listen<RepairProgress>('repair-progress', (event) => handler(event.payload)),
}

export function App() {
  useEffect(() => {
    void invoke<Pick<Settings, 'palette' | 'radius' | 'font'>>('fixer_look').then(applyLook).catch(() => {})
  }, [])
  return <main className="flex h-full flex-col bg-background text-foreground">
    <header data-tauri-drag-region className="flex shrink-0 items-center justify-between border-b border-border px-4 py-2">
      <span data-tauri-drag-region className="text-[10px] uppercase tracking-widest text-muted-foreground">Kryoto Auto-fixer</span>
      <button aria-label="Close" className="p-1 text-muted-foreground hover:text-foreground focus-visible:outline-2 focus-visible:outline-primary" onClick={() => void getCurrentWindow().close()}><X className="size-4" aria-hidden="true" /></button>
    </header>
    <div className="min-h-0 flex-1 overflow-auto px-5 py-4"><RepairPanel backend={backend} portable /></div>
  </main>
}
