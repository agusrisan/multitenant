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
