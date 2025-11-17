import { FormEvent } from 'react'
import { useForm, Link } from '@inertiajs/react'
import AuthLayout from '@/layouts/AuthLayout'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from '@/components/ui/card'
import { FormField } from '@/components/forms/FormField'
import { Alert, AlertDescription } from '@/components/ui/alert'
import { LoginProps } from '@/types'

export default function Login({ errors, flash }: LoginProps) {
  const { data, setData, post, processing } = useForm({
    email: '',
    password: '',
  })

  const handleSubmit = (e: FormEvent) => {
    e.preventDefault()
    post('/web/auth/login')
  }

  return (
    <AuthLayout>
      <Card>
        <CardHeader>
          <CardTitle>Login</CardTitle>
          <CardDescription>
            Enter your credentials to access your account
          </CardDescription>
        </CardHeader>

        <CardContent>
          {flash?.error && (
            <Alert variant="destructive" className="mb-4">
              <AlertDescription>{flash.error}</AlertDescription>
            </Alert>
          )}

          {flash?.success && (
            <Alert className="mb-4">
              <AlertDescription>{flash.success}</AlertDescription>
            </Alert>
          )}

          <form onSubmit={handleSubmit} className="space-y-4">
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

            <Button
              type="submit"
              className="w-full"
              disabled={processing}
            >
              {processing ? 'Logging in...' : 'Login'}
            </Button>
          </form>
        </CardContent>

        <CardFooter className="flex justify-center">
          <p className="text-sm text-gray-600">
            Don't have an account?{' '}
            <Link
              href="/web/auth/register"
              className="text-blue-600 hover:underline"
            >
              Register
            </Link>
          </p>
        </CardFooter>
      </Card>
    </AuthLayout>
  )
}
