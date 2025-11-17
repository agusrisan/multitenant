export const validateEmail = (email: string): string | null => {
  if (!email) return 'Email is required'
  const regex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/
  if (!regex.test(email)) return 'Invalid email format'
  return null
}

export const validatePassword = (password: string): string | null => {
  if (!password) return 'Password is required'
  if (password.length < 8) return 'Password must be at least 8 characters'
  return null
}

export const validatePasswordMatch = (
  password: string,
  confirmation: string
): string | null => {
  if (password !== confirmation) return 'Passwords do not match'
  return null
}

export const validateName = (name: string): string | null => {
  if (!name) return 'Name is required'
  if (name.length < 1) return 'Name cannot be empty'
  return null
}
