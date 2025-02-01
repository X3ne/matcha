import api, { User, UserProfile } from '@/api'
import { useToast } from '@/components/ui/use-toast'
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'
import { createContext } from 'react'

interface UserContextType {
  user: User | undefined
  userProfile: UserProfile | undefined
  isUserLoading: boolean
  isProfileLoading: boolean
  logout: () => void
  refreshUser: () => void
  refreshUserProfile: () => void
}

export const UserContext = createContext<UserContextType>({
  user: undefined,
  userProfile: undefined,
  isUserLoading: true,
  isProfileLoading: true,
  logout: () => {},
  refreshUser: () => {},
  refreshUserProfile: () => {}
})

export const UserProvider = ({ children }: { children: React.ReactNode }) => {
  const { toast } = useToast()
  const queryClient = useQueryClient()

  const { data: user, isLoading: isUserLoading } = useQuery({
    queryKey: ['user'],
    retry: false,
    queryFn: async () => (await api.v1.getMe({ credentials: 'include' })).data
  })

  const { data: userProfile, isLoading: isProfileLoading } = useQuery({
    queryKey: ['userProfile'],
    retry: false,
    queryFn: async () =>
      (await api.v1.getMyProfile({ credentials: 'include' })).data,
    enabled: !!user
  })

  const { mutate: logout } = useMutation({
    mutationFn: async () => await api.v1.logout({ credentials: 'include' }),
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

  const refreshUser = () => {
    queryClient.invalidateQueries({ queryKey: ['user'] })
  }

  const refreshUserProfile = () => {
    queryClient.invalidateQueries({ queryKey: ['userProfile'] })
  }

  return (
    <UserContext.Provider
      value={{
        user,
        isUserLoading,
        userProfile,
        isProfileLoading,
        logout,
        refreshUser,
        refreshUserProfile
      }}
    >
      {children}
    </UserContext.Provider>
  )
}
