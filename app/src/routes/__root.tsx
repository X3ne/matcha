import api from '@/api'
import { Layout } from '@/components/layout/layout' // Updated import path
import { Toaster } from '@/components/ui/toaster'
import { useUser } from '@/hooks/useUser'
import { GatewayProvider } from '@/providers/gatewayEventsProvider'
import { useQuery } from '@tanstack/react-query'
import { ReactQueryDevtools } from '@tanstack/react-query-devtools'
import {
  Outlet,
  createRootRoute,
  useLocation,
  useNavigate
} from '@tanstack/react-router'
import { TanStackRouterDevtools } from '@tanstack/router-devtools'
import { useEffect } from 'react'

const AllowedRoutes = ['/login', '/register', '/reset-password']

export const Route = createRootRoute({
  component: Root
})

function Root() {
  const { user, userProfile, isProfileLoading, isUserLoading } = useUser()
  const navigation = useNavigate()
  const location = useLocation()

  const controller = new AbortController()
  const signal = controller.signal

  setTimeout(() => {
    controller.abort()
  }, 2000)

  const { isError } = useQuery({
    queryKey: ['health'],
    queryFn: async () => {
      return await api.v1.health({
        signal
      })
    },
    retry: true
  })

  useEffect(() => {
    if (isError && location.pathname !== '/error') {
      navigation({ to: '/error' })
    } else if (!isError && location.pathname === '/error') {
      navigation({ to: '/' })
    }
  }, [location, isError, navigation])

  useEffect(() => {
    if (isError || isUserLoading) return

    const isActivationRoute = location.pathname.startsWith('/activation/')

    if (!user) {
      if (!AllowedRoutes.includes(location.pathname) && !isActivationRoute) {
        navigation({ to: '/login' })
      }
    } else {
      if (AllowedRoutes.includes(location.pathname)) {
        navigation({ to: '/search' })
      }
    }
  }, [user, isUserLoading, location.pathname, isError, navigation])

  useEffect(() => {
    if (isProfileLoading || isUserLoading) return

    if (user && !userProfile) {
      if (location.pathname !== '/onboarding') {
        navigation({ to: '/onboarding' })
      }
    }
  }, [user, userProfile, isProfileLoading, isUserLoading, location, navigation])

  return (
    <GatewayProvider>
      <Layout>
        <div className="mx-auto w-full">
          <Outlet />
        </div>
        <TanStackRouterDevtools />
        <ReactQueryDevtools initialIsOpen={false} />
        <Toaster />
      </Layout>
    </GatewayProvider>
  )
}
