import { FormEvent, useState } from 'react'
import { Link, router } from '@inertiajs/react'
import AppLayout from '@/layouts/AppLayout'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import ErrorMessage from '@/components/forms/ErrorMessage'

interface UserProfile {
  user_id: string
  name: string
  email: string
  bio?: string
  avatar_url?: string
  updated_at: string
}

interface EditProfileProps {
  profile: UserProfile
  errors?: Record<string, string>
}

export default function EditProfile({ profile, errors }: EditProfileProps) {
  const [formData, setFormData] = useState({
    name: profile.name || '',
    bio: profile.bio || '',
    avatar_url: profile.avatar_url || '',
  })
  const [loading, setLoading] = useState(false)
  const [validationErrors, setValidationErrors] = useState<Record<string, string>>({})

  const handleSubmit = (e: FormEvent) => {
    e.preventDefault()
    setValidationErrors({})

    // Client-side validation
    const errors: Record<string, string> = {}

    if (!formData.name.trim()) {
      errors.name = 'Name is required'
    }

    if (formData.bio && formData.bio.length > 500) {
      errors.bio = 'Bio cannot exceed 500 characters'
    }

    if (formData.avatar_url && !formData.avatar_url.match(/^https?:\/\/.+/)) {
      errors.avatar_url = 'Please enter a valid URL (starting with http:// or https://)'
    }

    if (Object.keys(errors).length > 0) {
      setValidationErrors(errors)
      return
    }

    setLoading(true)
    router.post('/web/user/profile/edit', formData, {
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
          <h1 className="text-3xl font-bold">Edit Profile</h1>
          <Link href="/web/user/profile">
            <Button variant="outline">Cancel</Button>
          </Link>
        </div>

        <form onSubmit={handleSubmit}>
          <Card>
            <CardHeader>
              <CardTitle>Personal Information</CardTitle>
            </CardHeader>
            <CardContent className="space-y-4">
              {/* Name Field */}
              <div>
                <Label htmlFor="name">Name *</Label>
                <Input
                  id="name"
                  type="text"
                  value={formData.name}
                  onChange={(e) => setFormData({ ...formData, name: e.target.value })}
                  placeholder="Enter your name"
                  required
                  aria-invalid={!!allErrors.name}
                  aria-describedby={allErrors.name ? 'name-error' : undefined}
                />
                {allErrors.name && (
                  <ErrorMessage id="name-error" message={allErrors.name} />
                )}
              </div>

              {/* Bio Field */}
              <div>
                <Label htmlFor="bio">
                  Bio
                  <span className="text-sm text-gray-500 ml-2">
                    ({formData.bio.length}/500 characters)
                  </span>
                </Label>
                <textarea
                  id="bio"
                  value={formData.bio}
                  onChange={(e) => setFormData({ ...formData, bio: e.target.value })}
                  placeholder="Tell us about yourself..."
                  rows={4}
                  maxLength={500}
                  className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                  aria-invalid={!!allErrors.bio}
                  aria-describedby={allErrors.bio ? 'bio-error' : undefined}
                />
                {allErrors.bio && <ErrorMessage id="bio-error" message={allErrors.bio} />}
              </div>

              {/* Avatar URL Field */}
              <div>
                <Label htmlFor="avatar_url">Avatar URL</Label>
                <Input
                  id="avatar_url"
                  type="url"
                  value={formData.avatar_url}
                  onChange={(e) => setFormData({ ...formData, avatar_url: e.target.value })}
                  placeholder="https://example.com/avatar.jpg"
                  aria-invalid={!!allErrors.avatar_url}
                  aria-describedby={allErrors.avatar_url ? 'avatar-error' : undefined}
                />
                {allErrors.avatar_url && (
                  <ErrorMessage id="avatar-error" message={allErrors.avatar_url} />
                )}
                {formData.avatar_url && (
                  <div className="mt-2">
                    <p className="text-sm text-gray-600 mb-2">Preview:</p>
                    <img
                      src={formData.avatar_url}
                      alt="Avatar preview"
                      className="w-20 h-20 rounded-full object-cover border-2 border-gray-200"
                      onError={(e) => {
                        e.currentTarget.style.display = 'none'
                      }}
                    />
                  </div>
                )}
              </div>

              {/* Submit Button */}
              <div className="flex gap-4 pt-4">
                <Button type="submit" disabled={loading}>
                  {loading ? 'Saving...' : 'Save Changes'}
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
