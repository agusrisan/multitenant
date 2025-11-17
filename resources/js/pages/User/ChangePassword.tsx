import { FormEvent, useState } from 'react'
import { Link, router } from '@inertiajs/react'
import AppLayout from '@/layouts/AppLayout'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import ErrorMessage from '@/components/forms/ErrorMessage'
import PasswordStrength from '@/components/forms/PasswordStrength'

interface ChangePasswordProps {
  errors?: Record<string, string>
}

export default function ChangePassword({ errors }: ChangePasswordProps) {
  const [formData, setFormData] = useState({
    current_password: '',
    new_password: '',
    new_password_confirmation: '',
  })
  const [loading, setLoading] = useState(false)
  const [validationErrors, setValidationErrors] = useState<Record<string, string>>({})
  const [showSuccess, setShowSuccess] = useState(false)

  const handleSubmit = (e: FormEvent) => {
    e.preventDefault()
    setValidationErrors({})
    setShowSuccess(false)

    // Client-side validation
    const errors: Record<string, string> = {}

    if (!formData.current_password) {
      errors.current_password = 'Current password is required'
    }

    if (!formData.new_password) {
      errors.new_password = 'New password is required'
    } else if (formData.new_password.length < 8) {
      errors.new_password = 'Password must be at least 8 characters'
    }

    if (!formData.new_password_confirmation) {
      errors.new_password_confirmation = 'Please confirm your new password'
    } else if (formData.new_password !== formData.new_password_confirmation) {
      errors.new_password_confirmation = 'Passwords do not match'
    }

    if (Object.keys(errors).length > 0) {
      setValidationErrors(errors)
      return
    }

    setLoading(true)
    router.post('/web/user/settings/password', formData, {
      onSuccess: () => {
        setShowSuccess(true)
        setFormData({
          current_password: '',
          new_password: '',
          new_password_confirmation: '',
        })
      },
      onFinish: () => setLoading(false),
      onError: (errors) => {
        setValidationErrors(errors)
      },
    })
  }

  const allErrors = { ...validationErrors, ...errors }

  return (
    <AppLayout>
      <div className="max-w-2xl mx-auto space-y-6 py-8">
        <div className="flex justify-between items-center">
          <h1 className="text-3xl font-bold">Change Password</h1>
          <Link href="/web/user/profile">
            <Button variant="outline">Back to Profile</Button>
          </Link>
        </div>

        {showSuccess && (
          <div className="bg-green-50 border border-green-200 text-green-800 px-4 py-3 rounded-md">
            <p className="font-medium">Password changed successfully!</p>
          </div>
        )}

        <form onSubmit={handleSubmit}>
          <Card>
            <CardHeader>
              <CardTitle>Change Your Password</CardTitle>
            </CardHeader>
            <CardContent className="space-y-4">
              {/* Current Password Field */}
              <div>
                <Label htmlFor="current_password">Current Password *</Label>
                <Input
                  id="current_password"
                  type="password"
                  value={formData.current_password}
                  onChange={(e) =>
                    setFormData({ ...formData, current_password: e.target.value })
                  }
                  placeholder="Enter your current password"
                  required
                  autoComplete="current-password"
                  aria-invalid={!!allErrors.current_password}
                  aria-describedby={
                    allErrors.current_password ? 'current-password-error' : undefined
                  }
                />
                {allErrors.current_password && (
                  <ErrorMessage
                    id="current-password-error"
                    message={allErrors.current_password}
                  />
                )}
              </div>

              {/* New Password Field */}
              <div>
                <Label htmlFor="new_password">New Password *</Label>
                <Input
                  id="new_password"
                  type="password"
                  value={formData.new_password}
                  onChange={(e) => setFormData({ ...formData, new_password: e.target.value })}
                  placeholder="Enter your new password"
                  required
                  autoComplete="new-password"
                  aria-invalid={!!allErrors.new_password}
                  aria-describedby={allErrors.new_password ? 'new-password-error' : undefined}
                />
                {allErrors.new_password && (
                  <ErrorMessage id="new-password-error" message={allErrors.new_password} />
                )}
                {formData.new_password && (
                  <PasswordStrength password={formData.new_password} />
                )}
              </div>

              {/* Confirm New Password Field */}
              <div>
                <Label htmlFor="new_password_confirmation">Confirm New Password *</Label>
                <Input
                  id="new_password_confirmation"
                  type="password"
                  value={formData.new_password_confirmation}
                  onChange={(e) =>
                    setFormData({ ...formData, new_password_confirmation: e.target.value })
                  }
                  placeholder="Confirm your new password"
                  required
                  autoComplete="new-password"
                  aria-invalid={!!allErrors.new_password_confirmation}
                  aria-describedby={
                    allErrors.new_password_confirmation
                      ? 'confirm-password-error'
                      : undefined
                  }
                />
                {allErrors.new_password_confirmation && (
                  <ErrorMessage
                    id="confirm-password-error"
                    message={allErrors.new_password_confirmation}
                  />
                )}
              </div>

              {/* Password Requirements */}
              <div className="bg-blue-50 border border-blue-200 rounded-md p-4">
                <p className="text-sm font-semibold text-blue-900 mb-2">
                  Password Requirements:
                </p>
                <ul className="text-sm text-blue-800 space-y-1 list-disc list-inside">
                  <li>At least 8 characters long</li>
                  <li>Include a mix of letters, numbers, and symbols (recommended)</li>
                  <li>Avoid using common words or personal information</li>
                </ul>
              </div>

              {/* Submit Button */}
              <div className="flex gap-4 pt-4">
                <Button type="submit" disabled={loading}>
                  {loading ? 'Changing Password...' : 'Change Password'}
                </Button>
                <Link href="/web/user/profile">
                  <Button type="button" variant="outline">
                    Cancel
                  </Button>
                </Link>
              </div>
            </CardContent>
          </Card>
        </form>
      </div>
    </AppLayout>
  )
}
