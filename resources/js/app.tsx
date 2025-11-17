/// <reference types="vite/client" />
import './bootstrap'
import { createRoot } from 'react-dom/client'
import { createInertiaApp } from '@inertiajs/react'
import '../css/app.css'

const appName = 'Auth App'

createInertiaApp({
  title: (title) => title ? `${title} - ${appName}` : appName,

  resolve: (name) => {
    const pages = import.meta.glob('./pages/**/*.tsx', { eager: true })
    const page = pages[`./pages/${name}.tsx`]

    if (!page) {
      throw new Error(`Page not found: ${name}`)
    }

    return page as any
  },

  setup({ el, App, props }) {
    const root = createRoot(el)
    root.render(<App {...props} />)
  },

  progress: {
    color: '#4B5563',
  },
})
