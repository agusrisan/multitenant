import { FormEvent } from 'react'
import { useForm, Link } from '@inertiajs/react'
import AuthLayout from '@/layouts/AuthLayout'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from '@/components/ui/card'
import { FormField } from '@/components/forms/FormField'
import { Alert, AlertDescription } from '@/components/ui/alert'
import { RegisterProps } from '@/types'

export default function Register({ errors, flash }: RegisterProps) {
  const { data, setData, post, processing } = useForm({
    name: '',
    email: '',
    password: '',
    password_confirmation: '',
  })

  const handleSubmit = (e: FormEvent) => {
    e.preventDefault()
    post('/web/auth/register')
  }

  return (
    <AuthLayout>
      <Card>
        <CardHeader>
          <CardTitle>Create Account</CardTitle>
          <CardDescription>
            Sign up to get started
          </CardDescription>
        </CardHeader>

        <CardContent>
          {flash?.error && (
            <Alert variant="destructive" className="mb-4">
              <AlertDescription>{flash.error}</AlertDescription>
            </Alert>
          )}

          <form onSubmit={handleSubmit} className="space-y-4">
            <FormField
              label="Name"
              name="name"
              type="text"
              value={data.name}
              onChange={(e) => setData('name', e.target.value)}
              error={errors?.name?.[0]}
              required
            />

            <FormField
              label="Email"
              name="email"
              type="email"
              value={data.email}
              onChange={(e) => setData('email', e.target.value)}
              error={errors?.email?.[0]}
              required
            />

            <FormField
              label="Password"
              name="password"
              type="password"
              value={data.password}
              onChange={(e) => setData('password', e.target.value)}
              error={errors?.password?.[0]}
              required
            />

            <FormField
              label="Confirm Password"
              name="password_confirmation"
              type="password"
              value={data.password_confirmation}
              onChange={(e) => setData('password_confirmation', e.target.value)}
              error={errors?.password_confirmation?.[0]}
              required
            />

            <div className="text-sm text-gray-600">
              <p>Password must:</p>
              <ul className="list-disc list-inside mt-1 space-y-1">
                <li>Be at least 8 characters long</li>
                <li>Match the confirmation</li>
              </ul>
            </div>

            <Button
              type="submit"
              className="w-full"
              disabled={processing}
            >
              {processing ? 'Creating account...' : 'Register'}
            </Button>
          </form>
        </CardContent>

        <CardFooter className="flex justify-center">
          <p className="text-sm text-gray-600">
            Already have an account?{' '}
            <Link
              href="/web/auth/login"
              className="text-blue-600 hover:underline"
            >
              Login
            </Link>
          </p>
        </CardFooter>
      </Card>
    </AuthLayout>
  )
}
