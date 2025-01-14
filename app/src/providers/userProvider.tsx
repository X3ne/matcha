import api, { User, UserProfile } from '@/api'
import { useToast } from '@/components/ui/use-toast'
import { useMutation, useQuery } from '@tanstack/react-query'
import { createContext } from 'react'

export const UserContext = createContext({
  user: undefined as unknown as User | undefined,
  userProfile: undefined as unknown as UserProfile | undefined,
  isUserLoading: true,
  isProfileLoading: true,
  logout: () => {}
})

export const UserProvider = ({ children }: { children: React.ReactNode }) => {
  const { toast } = useToast()

  const { data: user, isLoading: isUserLoading } = useQuery({
    queryKey: ['user'],
    retry: false,
    queryFn: async () =>
      (
        await api.v1.getMe({
          credentials: 'include'
        })
      ).data
  })

  const { data: userProfile, isLoading: isProfileLoading } = useQuery({
    queryKey: ['userProfile'],
    retry: false,
    queryFn: async () =>
      (
        await api.v1.getMyProfile({
          credentials: 'include'
        })
      ).data,
    enabled: !!user
  })

  const { mutate: logout } = useMutation({
    mutationFn: async () =>
      await api.v1.logout({
        credentials: 'include'
      }),
    onSuccess: () => {
      toast({
        title: 'Logged out',
        description: 'You have been logged out',
        variant: 'default'
      })

      window.location.href = '/auth/login'
    },
    onError: (err) => {
      toast({
        title: 'An error occurred',
        description: 'Failed to logout',
        variant: 'destructive'
      })

      console.error(err)
    }
  })

  return (
    <UserContext.Provider
      value={{ user, isUserLoading, userProfile, isProfileLoading, logout }}
    >
      {children}
    </UserContext.Provider>
  )
}
