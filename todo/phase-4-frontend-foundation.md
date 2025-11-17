# Phase 4: Frontend Foundation (React 19 + Vite + Tailwind 4)

**Status**: 🔴 Not Started
**Priority**: 🔥 HIGH
**Estimated Time**: 6-8 hours
**Dependencies**: Phase 1 (Project Setup)

## Overview
Frontend infrastructure: React 19, Inertia.js, Vite build system, Tailwind CSS 4, shadcn/ui components, TypeScript configuration, and layout system.

---

## 1. Core Frontend Setup

### 1.1 Install Dependencies
- [ ] **Run npm install** 📁 `resources/`
  - **Priority**: High
  - **Complexity**: Simple (5 min)
  - **Dependencies**: Requires package.json from Phase 1
  - **Command**: `cd resources && npm install`
  - **Verification**: Check node_modules/ created

### 1.2 Directory Structure
- [ ] **Create frontend directories** 📁 `resources/js/`
  - **Priority**: High
  - **Complexity**: Simple (5 min)
  - **Dependencies**: None
  - **Directories**:
    ```
    resources/
    ├── css/
    │   └── app.css
    ├── js/
    │   ├── app.tsx
    │   ├── bootstrap.ts
    │   ├── layouts/
    │   ├── components/
    │   └── pages/
    └── public/  (static assets)
    ```

---

## 2. Inertia.js Setup

### 2.1 Bootstrap File
- [ ] **Create bootstrap.ts** 📁 `resources/js/bootstrap.ts`
  - **Priority**: High
  - **Complexity**: Medium (45 min)
  - **Dependencies**: None
  - **Purpose**: Initialize Inertia.js, Axios, global config
  - **Content**:
    ```typescript
    import axios from 'axios'
    import { router } from '@inertiajs/react'

    // Configure Axios
    axios.defaults.headers.common['X-Requested-With'] = 'XMLHttpRequest'
    axios.defaults.withCredentials = true

    // CSRF token handling
    const token = document.head.querySelector('meta[name="csrf-token"]')
    if (token) {
      axios.defaults.headers.common['X-CSRF-Token'] = (token as HTMLMetaElement).content
    }

    // Inertia progress bar (optional)
    import NProgress from 'nprogress'
    import 'nprogress/nprogress.css'

    router.on('start', () => NProgress.start())
    router.on('finish', () => NProgress.done())
    ```
  - **Features**:
    - Axios with CSRF token
    - Credentials included for cookies
    - Progress bar for navigation
  - **Optional Deps**: `nprogress` for loading indicator

### 2.2 App Entry Point
- [ ] **Create app.tsx** 📁 `resources/js/app.tsx`
  - **Priority**: High
  - **Complexity**: Medium (1 hour)
  - **Dependencies**: Requires bootstrap.ts
  - **Tech**: React 19, Inertia.js
  - **Content**:
    ```typescript
    import './bootstrap'
    import { createRoot } from 'react-dom/client'
    import { createInertiaApp } from '@inertiajs/react'
    import { resolvePageComponent } from 'laravel-vite-plugin/inertia-helpers'
    import '../css/app.css'

    const appName = 'Auth App'

    createInertiaApp({
      title: (title) => title ? `${title} - ${appName}` : appName,

      resolve: (name) => resolvePageComponent(
        `./pages/${name}.tsx`,
        import.meta.glob('./pages/**/*.tsx')
      ),

      setup({ el, App, props }) {
        const root = createRoot(el)
        root.render(<App {...props} />)
      },

      progress: {
        color: '#4B5563',
      },
    })
    ```
  - **Page Resolution**: Dynamic imports from `./pages/`
  - **Title Template**: Automatic page title formatting
  - **SSR**: Not enabled yet (can add in Phase 7)

### 2.3 Vite Entry HTML
- [ ] **Create index.html** 📁 `resources/index.html`
  - **Priority**: Medium
  - **Complexity**: Simple (20 min)
  - **Dependencies**: None
  - **Purpose**: Vite dev server entry point
  - **Content**:
    ```html
    <!DOCTYPE html>
    <html lang="en">
      <head>
        <meta charset="UTF-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <meta name="csrf-token" content="" />
        <title>Auth App</title>
      </head>
      <body>
        <div id="app" data-page=""></div>
        <script type="module" src="/js/app.tsx"></script>
      </body>
    </html>
    ```
  - **Notes**: Rust backend will replace `data-page` with Inertia page data

---

## 3. Tailwind CSS 4 Configuration

### 3.1 Tailwind Setup
- [ ] **Verify tailwind.config.ts exists** 📁 `resources/tailwind.config.ts`
  - **Priority**: High
  - **Complexity**: Simple (10 min)
  - **Dependencies**: Created in Phase 1
  - **Tech**: Tailwind CSS 4
  - **Verification**: Check content paths include `./js/**/*.{ts,tsx}`

### 3.2 Base Styles
- [ ] **Verify app.css exists** 📁 `resources/css/app.css`
  - **Priority**: High
  - **Complexity**: Simple (10 min)
  - **Dependencies**: Created in Phase 1
  - **Content Check**:
    ```css
    @import "tailwindcss";

    @layer base {
      :root {
        --background: 0 0% 100%;
        --foreground: 222.2 84% 4.9%;
        --card: 0 0% 100%;
        --card-foreground: 222.2 84% 4.9%;
        --popover: 0 0% 100%;
        --popover-foreground: 222.2 84% 4.9%;
        --primary: 222.2 47.4% 11.2%;
        --primary-foreground: 210 40% 98%;
        --secondary: 210 40% 96.1%;
        --secondary-foreground: 222.2 47.4% 11.2%;
        --muted: 210 40% 96.1%;
        --muted-foreground: 215.4 16.3% 46.9%;
        --accent: 210 40% 96.1%;
        --accent-foreground: 222.2 47.4% 11.2%;
        --destructive: 0 84.2% 60.2%;
        --destructive-foreground: 210 40% 98%;
        --border: 214.3 31.8% 91.4%;
        --input: 214.3 31.8% 91.4%;
        --ring: 222.2 84% 4.9%;
        --radius: 0.5rem;
      }

      .dark {
        --background: 222.2 84% 4.9%;
        --foreground: 210 40% 98%;
        /* ... dark mode variables */
      }
    }

    @layer base {
      * {
        @apply border-border;
      }
      body {
        @apply bg-background text-foreground;
      }
    }
    ```
  - **Purpose**: shadcn/ui compatible CSS variables

---

## 4. shadcn/ui Setup

### 4.1 shadcn Components Directory
- [ ] **Create components directory** 📁 `resources/js/components/`
  - **Priority**: High
  - **Complexity**: Simple (5 min)
  - **Dependencies**: None
  - **Structure**:
    ```
    components/
    ├── ui/           # shadcn components
    ├── forms/        # form-specific components
    └── layout/       # layout components
    ```

### 4.2 shadcn Utils
- [ ] **Create lib/utils.ts** 📁 `resources/js/lib/utils.ts`
  - **Priority**: High
  - **Complexity**: Simple (15 min)
  - **Dependencies**: None
  - **Content**:
    ```typescript
    import { type ClassValue, clsx } from "clsx"
    import { twMerge } from "tailwind-merge"

    export function cn(...inputs: ClassValue[]) {
      return twMerge(clsx(inputs))
    }
    ```
  - **Dependencies**: Install `clsx` and `tailwind-merge`
  - **Command**: `npm install clsx tailwind-merge`
  - **Purpose**: Merge Tailwind classes intelligently

### 4.3 Install Core shadcn Components
- [ ] **Add Button component** 📁 `resources/js/components/ui/button.tsx`
  - **Priority**: High
  - **Complexity**: Simple (20 min)
  - **Dependencies**: Requires utils.ts
  - **Method**: Manual copy from shadcn/ui docs or use CLI
  - **CLI**: `npx shadcn-ui@latest add button` (may need config)
  - **Usage**: Login/Register forms

- [ ] **Add Input component** 📁 `resources/js/components/ui/input.tsx`
  - **Priority**: High
  - **Complexity**: Simple (20 min)
  - **Dependencies**: None
  - **Purpose**: Form inputs (email, password)

- [ ] **Add Label component** 📁 `resources/js/components/ui/label.tsx`
  - **Priority**: High
  - **Complexity**: Simple (15 min)
  - **Dependencies**: None
  - **Purpose**: Form labels

- [ ] **Add Card component** 📁 `resources/js/components/ui/card.tsx`
  - **Priority**: Medium
  - **Complexity**: Simple (20 min)
  - **Dependencies**: None
  - **Purpose**: Login/Register form containers

- [ ] **Add Alert component** 📁 `resources/js/components/ui/alert.tsx`
  - **Priority**: Medium
  - **Complexity**: Simple (20 min)
  - **Dependencies**: None
  - **Purpose**: Error/success messages

### 4.4 shadcn Configuration
- [ ] **Create components.json** 📁 `resources/components.json`
  - **Priority**: Medium
  - **Complexity**: Simple (15 min)
  - **Dependencies**: None
  - **Purpose**: shadcn CLI configuration
  - **Content**:
    ```json
    {
      "$schema": "https://ui.shadcn.com/schema.json",
      "style": "default",
      "rsc": false,
      "tsx": true,
      "tailwind": {
        "config": "tailwind.config.ts",
        "css": "css/app.css",
        "baseColor": "slate",
        "cssVariables": true
      },
      "aliases": {
        "components": "@/components",
        "utils": "@/lib/utils"
      }
    }
    ```
  - **Features**: Enable path aliases, CSS variables

---

## 5. Layout System

### 5.1 Layouts Directory
- [ ] **Create layouts directory** 📁 `resources/js/layouts/`
  - **Priority**: High
  - **Complexity**: Simple (5 min)
  - **Dependencies**: None

### 5.2 Auth Layout
- [ ] **Create AuthLayout.tsx** 📁 `resources/js/layouts/AuthLayout.tsx`
  - **Priority**: High
  - **Complexity**: Medium (1 hour)
  - **Dependencies**: shadcn components
  - **Purpose**: Layout for login/register pages
  - **Structure**:
    ```typescript
    import { ReactNode } from 'react'

    interface AuthLayoutProps {
      children: ReactNode
    }

    export default function AuthLayout({ children }: AuthLayoutProps) {
      return (
        <div className="min-h-screen flex items-center justify-center bg-gray-50">
          <div className="w-full max-w-md p-6">
            <div className="text-center mb-8">
              <h1 className="text-3xl font-bold text-gray-900">Auth App</h1>
              <p className="text-gray-600 mt-2">Welcome back</p>
            </div>
            {children}
          </div>
        </div>
      )
    }
    ```
  - **Features**: Centered card, logo, responsive
  - **Styling**: Tailwind CSS 4

### 5.3 App Layout
- [ ] **Create AppLayout.tsx** 📁 `resources/js/layouts/AppLayout.tsx`
  - **Priority**: High
  - **Complexity**: Complex (2 hours)
  - **Dependencies**: shadcn components, Inertia
  - **Purpose**: Layout for authenticated pages (dashboard, profile)
  - **Structure**:
    ```typescript
    import { ReactNode } from 'react'
    import { Link, usePage } from '@inertiajs/react'
    import { PageProps } from '@/types'

    interface AppLayoutProps {
      children: ReactNode
    }

    export default function AppLayout({ children }: AppLayoutProps) {
      const { auth } = usePage<PageProps>().props

      return (
        <div className="min-h-screen bg-gray-100">
          {/* Header */}
          <header className="bg-white shadow">
            <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
              <div className="flex justify-between h-16 items-center">
                <Link href="/web/dashboard" className="text-xl font-bold">
                  Auth App
                </Link>
                <nav className="flex items-center gap-4">
                  <span className="text-sm text-gray-600">
                    {auth.user.email}
                  </span>
                  <Link
                    href="/web/auth/logout"
                    method="post"
                    as="button"
                    className="text-sm text-red-600 hover:text-red-700"
                  >
                    Logout
                  </Link>
                </nav>
              </div>
            </div>
          </header>

          {/* Main Content */}
          <main className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
            {children}
          </main>
        </div>
      )
    }
    ```
  - **Features**: Header, nav, logout button, user info
  - **Props**: auth.user from Inertia shared props

---

## 6. TypeScript Configuration

### 6.1 Type Definitions
- [ ] **Create types/index.ts** 📁 `resources/js/types/index.ts`
  - **Priority**: High
  - **Complexity**: Medium (45 min)
  - **Dependencies**: None
  - **Purpose**: Shared TypeScript types
  - **Content**:
    ```typescript
    export interface User {
      id: string
      email: string
      name: string
      email_verified: boolean
      is_active: boolean
      created_at: string
      updated_at: string
    }

    export interface Session {
      id: string
      user_id: string
      expires_at: string
    }

    export interface PageProps {
      auth: {
        user: User
      }
      flash?: {
        success?: string
        error?: string
      }
      errors?: Record<string, string[]>
    }

    export interface InertiaSharedProps {
      auth: {
        user: User | null
      }
    }
    ```
  - **Usage**: Type-safe Inertia props

### 6.2 Inertia Types
- [ ] **Create inertia.d.ts** 📁 `resources/js/types/inertia.d.ts`
  - **Priority**: Medium
  - **Complexity**: Simple (20 min)
  - **Dependencies**: @inertiajs/react
  - **Purpose**: Global Inertia type augmentation
  - **Content**:
    ```typescript
    import { PageProps as InertiaPageProps } from '@inertiajs/core'
    import { PageProps as AppPageProps } from './index'

    declare module '@inertiajs/core' {
      interface PageProps extends InertiaPageProps, AppPageProps {}
    }
    ```
  - **Benefit**: Auto-complete for usePage().props

---

## 7. Pages Directory Setup

### 7.1 Pages Structure
- [ ] **Create pages directories** 📁 `resources/js/pages/`
  - **Priority**: High
  - **Complexity**: Simple (5 min)
  - **Dependencies**: None
  - **Structure**:
    ```
    pages/
    ├── Auth/
    │   ├── Login.tsx
    │   └── Register.tsx
    └── Dashboard/
        └── Index.tsx
    ```
  - **Notes**: Inertia resolves pages by path (Auth/Login → Auth::Login)

---

## 8. Form Utilities

### 8.1 Form Helper Hook
- [ ] **Create useForm hook** 📁 `resources/js/hooks/useForm.ts`
  - **Priority**: Medium
  - **Complexity**: Medium (1 hour)
  - **Dependencies**: @inertiajs/react
  - **Purpose**: Form state management with Inertia
  - **Content**:
    ```typescript
    import { useForm as useInertiaForm } from '@inertiajs/react'

    export function useForm<T>(initialValues: T) {
      const form = useInertiaForm(initialValues)

      const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        form.setData(e.target.name as keyof T, e.target.value as any)
      }

      return {
        ...form,
        handleChange,
      }
    }
    ```
  - **Features**: Type-safe form handling
  - **Alternative**: Use Inertia's built-in useForm directly

### 8.2 Form Components
- [ ] **Create FormField component** 📁 `resources/js/components/forms/FormField.tsx`
  - **Priority**: Medium
  - **Complexity**: Medium (45 min)
  - **Dependencies**: shadcn Input, Label
  - **Purpose**: Reusable form field with error display
  - **Structure**:
    ```typescript
    import { Input } from '@/components/ui/input'
    import { Label } from '@/components/ui/label'

    interface FormFieldProps {
      label: string
      name: string
      type?: string
      value: string
      onChange: (e: React.ChangeEvent<HTMLInputElement>) => void
      error?: string
      required?: boolean
    }

    export function FormField({
      label,
      name,
      type = 'text',
      value,
      onChange,
      error,
      required
    }: FormFieldProps) {
      return (
        <div className="space-y-2">
          <Label htmlFor={name}>
            {label} {required && <span className="text-red-500">*</span>}
          </Label>
          <Input
            id={name}
            name={name}
            type={type}
            value={value}
            onChange={onChange}
            className={error ? 'border-red-500' : ''}
            required={required}
          />
          {error && (
            <p className="text-sm text-red-500">{error}</p>
          )}
        </div>
      )
    }
    ```
  - **Features**: Error styling, required indicator

---

## 9. Build Configuration

### 9.1 Vite Config Update
- [ ] **Verify vite.config.ts** 📁 `resources/vite.config.ts`
  - **Priority**: High
  - **Complexity**: Simple (15 min)
  - **Dependencies**: Created in Phase 1
  - **Required Config**:
    ```typescript
    import { defineConfig } from 'vite'
    import react from '@vitejs/plugin-react'
    import path from 'path'

    export default defineConfig({
      plugins: [react()],
      resolve: {
        alias: {
          '@': path.resolve(__dirname, './js'),
        },
      },
      build: {
        manifest: true,
        outDir: 'dist',
        rollupOptions: {
          input: 'js/app.tsx',
        },
      },
      server: {
        origin: 'http://localhost:5173',
      },
    })
    ```
  - **Features**: Path aliases, manifest for production, dev server config

### 9.2 Environment Variables
- [ ] **Create .env.local** 📁 `resources/.env.local`
  - **Priority**: Low
  - **Complexity**: Simple (5 min)
  - **Dependencies**: None
  - **Purpose**: Frontend-specific env vars (if needed)
  - **Notes**: Vite uses VITE_ prefix for public vars

---

## Validation Checklist

### Setup:
- [ ] `npm install` completes without errors
- [ ] `npm run dev` starts Vite dev server on port 5173
- [ ] TypeScript compilation has no errors
- [ ] Tailwind CSS compiles and processes classes

### Components:
- [ ] shadcn components render correctly
- [ ] Button, Input, Label components work
- [ ] CSS variables apply correctly
- [ ] Dark mode variables defined (for future use)

### Inertia:
- [ ] Inertia app initializes without errors
- [ ] Page components can be resolved from pages/
- [ ] Progress bar shows on navigation
- [ ] CSRF token loaded from meta tag

### Layouts:
- [ ] AuthLayout renders centered card
- [ ] AppLayout renders header + main content
- [ ] Logout button uses Inertia Link with method="post"

### Types:
- [ ] TypeScript autocomplete works for props
- [ ] usePage().props has type checking
- [ ] No TypeScript errors in components

---

## Architecture Notes

### Frontend Architecture:
- **React 19**: Latest features (improved Suspense, Server Components ready)
- **Inertia.js**: Server-driven SPA (no API needed for web routes)
- **Vite**: Fast HMR, optimized builds
- **Tailwind 4**: New @import syntax, better performance
- **shadcn/ui**: Unstyled components, full customization

### File Organization:
```
resources/
├── js/
│   ├── app.tsx          # Entry point
│   ├── bootstrap.ts     # Axios, Inertia config
│   ├── types/           # TypeScript definitions
│   ├── hooks/           # Custom hooks
│   ├── lib/             # Utilities
│   ├── components/      # Reusable components
│   │   ├── ui/          # shadcn components
│   │   └── forms/       # Form components
│   ├── layouts/         # Page layouts
│   └── pages/           # Inertia pages
└── css/
    └── app.css          # Tailwind entry
```

### Integration with Backend:
- Backend serves HTML with Inertia data in `data-page`
- Vite manifest.json used in production for asset URLs
- Dev mode: Vite dev server (HMR), backend proxies assets
- Prod mode: Built assets in resources/dist/, served by Rust

---

## Next Phase
➡️ **Phase 5: Auth Frontend** - Login/Register pages with forms, validation, and Inertia integration

## Technical Notes
- React 19 is in RC (use with caution in production)
- Tailwind 4 requires PostCSS 8+
- Inertia v2 has better TypeScript support
- shadcn components are unstyled (full control over styling)
- Vite manifest required for production asset URLs
