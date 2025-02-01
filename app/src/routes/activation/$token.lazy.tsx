import api from '@/api'
import { useToast } from '@/components/ui/use-toast'
import { useMutation } from '@tanstack/react-query'
import { useParams, useNavigate } from '@tanstack/react-router'
import { createLazyFileRoute } from '@tanstack/react-router'
import { useEffect } from 'react'

export const Route = createLazyFileRoute('/activation/$token')({
  component: Activation
})

function Activation() {
  const { toast } = useToast()
  const navigate = useNavigate()
  const { token } = useParams({ from: '/activation/$token' })

  const { mutate: activation } = useMutation({
    mutationFn: () => api.v1.activateAccount({ token }),
    onSuccess: () => {
      toast({
        title: 'Account Activated',
        description:
          'Your account has been successfully activated. You can now log in.',
        variant: 'default'
      })
      navigate({ to: '/login' })
    },
    onError: (error: any) => {
      const message =
        error?.response?.data?.message || 'Failed to activate your account.'
      toast({
        title: 'Activation Failed',
        description: message,
        variant: 'destructive'
      })
      navigate({ to: '/register' })
    }
  })

  useEffect(() => {
    if (token) {
      activation()
    } else {
      toast({
        title: 'Invalid Activation Link',
        description: 'No activation token provided.',
        variant: 'destructive'
      })
      navigate({ to: '/register' })
    }
  }, [token, activation, toast, navigate])

  return (
    <div className="activation-page">
      <p>Activating your account...</p>
    </div>
  )
}
