import api from '@/api'
import { ResetPassword } from '@/api/spec'
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
import { useToast } from '@/components/ui/use-toast'
import { useMutation } from '@tanstack/react-query'
import { createLazyFileRoute, useSearch } from '@tanstack/react-router'
import React, { useState } from 'react'

export const Route = createLazyFileRoute('/reset-password/')({
  component: RouteComponent
})

function RouteComponent() {
  const { toast } = useToast()

  const { email, token } = useSearch({ from: '/reset-password/' })

  const [password, setPassword] = useState('')
  const [confirmPassword, setConfirmPassword] = useState('')

  const { mutate: handlePasswordReset, isPending: isResetLoading } =
    useMutation<void, Error, ResetPassword>({
      mutationFn: async (data: ResetPassword) => {
        await api.v1.resetPassword(data)
      },
      onSuccess: () => {
        toast({
          title: 'Password Reset Successful',
          description: 'Your password has been updated successfully.'
        })
        window.location.href = '/login'
      },
      onError: (err) => {
        console.error('Password reset error:', err)
        toast({
          title: 'Error',
          description: 'Failed to reset password. Please try again.',
          variant: 'destructive'
        })
      }
    })

  const onSubmitReset = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault()

    if (password !== confirmPassword) {
      toast({
        title: 'Error',
        description: 'Passwords do not match.',
        variant: 'destructive'
      })
      return
    }

    handlePasswordReset({
      email,
      password,
      confirm_password: confirmPassword,
      token
    })
  }

  return (
    <div className="flex min-h-screen items-center justify-center">
      <Card className="w-full max-w-md">
        <CardHeader className="text-center">
          <CardTitle className="text-xl">Reset Password</CardTitle>
          <CardDescription>
            Enter your details to reset your password
          </CardDescription>
        </CardHeader>
        <CardContent>
          <form onSubmit={onSubmitReset} className="grid gap-6">
            <div className="grid gap-2">
              <Label htmlFor="password">Password</Label>
              <Input
                id="password"
                type="password"
                required
                value={password}
                onChange={(e) => setPassword(e.target.value)}
              />
            </div>

            <div className="grid gap-2">
              <Label htmlFor="confirmPassword">Confirm Password</Label>
              <Input
                id="confirmPassword"
                type="password"
                required
                value={confirmPassword}
                onChange={(e) => setConfirmPassword(e.target.value)}
              />
            </div>

            <Button type="submit" className="w-full" disabled={isResetLoading}>
              {isResetLoading ? 'Submitting...' : 'Reset Password'}
            </Button>
          </form>
        </CardContent>
      </Card>
    </div>
  )
}
