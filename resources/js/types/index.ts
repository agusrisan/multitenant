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
  [key: string]: any
}

export interface InertiaSharedProps {
  auth: {
    user: User | null
  }
}
