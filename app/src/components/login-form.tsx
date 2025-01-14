import api from '@/api'
import { Button } from '@/components/ui/button'
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle
} from '@/components/ui/card'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { cn } from '@/lib/utils'
import { useMutation } from '@tanstack/react-query'
import { Link } from '@tanstack/react-router'
import React, { useState } from 'react'

// import { Si42 } from 'react-icons/si'

export function LoginForm({
  className,
  ...props
}: React.ComponentPropsWithoutRef<'div'>) {
  const [username, setUsername] = useState('')
  const [password, setPassword] = useState('')

  const { mutate: handle42Login, isPending: is42Loading } = useMutation<
    void,
    Error,
    void
  >({
    mutationFn: async () => {
      const res = await api.v1.login42()
      window.open(res.data.url, 'auth42', 'width=600,height=700')
    },
    onError: (err) => {
      console.error('42 login error:', err)
    }
  })

  const { mutate: handleCredentialsLogin, isPending: isLoginLoading } =
    useMutation<void, Error, { username: string; password: string }>({
      mutationFn: async (credentials) => {
        await api.v1.login(credentials)
      },
      onSuccess: () => {
        window.location.href = '/search'
      },
      onError: (err) => {
        console.log('Login error:', err)
      }
    })

  const onSubmitCredentials = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault()
    handleCredentialsLogin({ username, password })
  }

  return (
    <div className={cn('flex flex-col gap-6', className)} {...props}>
      <Card>
        <CardHeader className="text-center">
          <CardTitle className="text-xl">Welcome back</CardTitle>
          <CardDescription>Login with your 42 account</CardDescription>
        </CardHeader>
        <CardContent>
          <div className="grid gap-6">
            <div>
              <Button
                variant="outline"
                className="w-full"
                onClick={() => handle42Login()}
                disabled={is42Loading}
              >
                Continue with 42
              </Button>
            </div>
          </div>

          {/* Divider */}
          <div className="relative my-6 text-center text-sm after:absolute after:inset-0 after:top-1/2 after:z-0 after:flex after:items-center after:border-t after:border-border">
            <span className="relative z-10 bg-background px-2 text-muted-foreground">
              Or continue with
            </span>
          </div>

          {/* Username/Password Form */}
          <form onSubmit={onSubmitCredentials}>
            <div className="grid gap-6">
              <div className="grid gap-2">
                <Label htmlFor="username">Username</Label>
                <Input
                  id="username"
                  type="text"
                  required
                  value={username}
                  onChange={(e) => setUsername(e.target.value)}
                />
              </div>

              <div className="grid gap-2">
                <div className="flex items-center">
                  <Label htmlFor="password">Password</Label>
                  <a
                    href="#"
                    className="ml-auto text-sm underline-offset-4 hover:underline"
                  >
                    Forgot your password?
                  </a>
                </div>
                <Input
                  id="password"
                  type="password"
                  required
                  value={password}
                  onChange={(e) => setPassword(e.target.value)}
                />
              </div>

              <Button
                type="submit"
                className="w-full"
                disabled={isLoginLoading}
              >
                Login
              </Button>

              <div className="text-center text-sm">
                Don&apos;t have an account?{' '}
                <Link to="/register" className="underline underline-offset-4">
                  Sign up
                </Link>
              </div>
            </div>
          </form>
        </CardContent>
      </Card>
    </div>
  )
}
