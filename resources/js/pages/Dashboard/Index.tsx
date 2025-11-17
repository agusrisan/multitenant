import { PageProps } from '@/types'
import AppLayout from '@/layouts/AppLayout'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'

export default function Dashboard({ auth }: PageProps) {
  return (
    <AppLayout>
      <div className="space-y-6">
        <div>
          <h1 className="text-3xl font-bold text-gray-900">Dashboard</h1>
          <p className="text-gray-600 mt-2">
            Welcome back, {auth.user.name}!
          </p>
        </div>

        <div className="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
          <Card>
            <CardHeader>
              <CardTitle>Account Info</CardTitle>
              <CardDescription>Your account details</CardDescription>
            </CardHeader>
            <CardContent className="space-y-2">
              <div>
                <span className="font-semibold">Email:</span>{' '}
                {auth.user.email}
              </div>
              <div>
                <span className="font-semibold">Name:</span>{' '}
                {auth.user.name}
              </div>
              <div>
                <span className="font-semibold">Status:</span>{' '}
                {auth.user.is_active ? (
                  <span className="text-green-600">Active</span>
                ) : (
                  <span className="text-red-600">Inactive</span>
                )}
              </div>
              <div>
                <span className="font-semibold">Email Verified:</span>{' '}
                {auth.user.email_verified ? (
                  <span className="text-green-600">Yes</span>
                ) : (
                  <span className="text-yellow-600">Pending</span>
                )}
              </div>
            </CardContent>
          </Card>

          <Card>
            <CardHeader>
              <CardTitle>Quick Stats</CardTitle>
              <CardDescription>Your activity overview</CardDescription>
            </CardHeader>
            <CardContent>
              <p className="text-gray-600">
                Member since {new Date(auth.user.created_at).toLocaleDateString()}
              </p>
            </CardContent>
          </Card>
        </div>
      </div>
    </AppLayout>
  )
}
