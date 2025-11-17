import { Link } from '@inertiajs/react'
import AppLayout from '@/layouts/AppLayout'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Button } from '@/components/ui/button'

interface UserProfile {
  user_id: string
  name: string
  email: string
  bio?: string
  avatar_url?: string
  updated_at: string
}

interface ProfileProps {
  profile: UserProfile
}

export default function Profile({ profile }: ProfileProps) {
  return (
    <AppLayout>
      <div className="max-w-2xl mx-auto space-y-6 py-8">
        <div className="flex justify-between items-center">
          <h1 className="text-3xl font-bold">Profile</h1>
          <Link href="/web/user/profile/edit">
            <Button>Edit Profile</Button>
          </Link>
        </div>

        <Card>
          <CardHeader>
            <CardTitle>Personal Information</CardTitle>
          </CardHeader>
          <CardContent className="space-y-4">
            {profile.avatar_url && (
              <div className="flex justify-center">
                <img
                  src={profile.avatar_url}
                  alt={profile.name}
                  className="w-24 h-24 rounded-full object-cover border-2 border-gray-200"
                />
              </div>
            )}
            <div>
              <label className="font-semibold text-sm text-gray-600">Name:</label>
              <p className="text-lg">{profile.name}</p>
            </div>
            <div>
              <label className="font-semibold text-sm text-gray-600">Email:</label>
              <p className="text-lg">{profile.email}</p>
            </div>
            {profile.bio && (
              <div>
                <label className="font-semibold text-sm text-gray-600">Bio:</label>
                <p className="text-gray-700 whitespace-pre-wrap">{profile.bio}</p>
              </div>
            )}
            {!profile.bio && (
              <div>
                <label className="font-semibold text-sm text-gray-600">Bio:</label>
                <p className="text-gray-400 italic">No bio added yet</p>
              </div>
            )}
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle>Account Settings</CardTitle>
          </CardHeader>
          <CardContent>
            <Link href="/web/user/settings/password">
              <Button variant="outline" className="w-full sm:w-auto">
                Change Password
              </Button>
            </Link>
          </CardContent>
        </Card>
      </div>
    </AppLayout>
  )
}
