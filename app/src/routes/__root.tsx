import api from '@/api'
import { Toaster } from '@/components/ui/toaster'
import { useUser } from '@/hooks/useUser'
import { useQuery } from '@tanstack/react-query'
import { ReactQueryDevtools } from '@tanstack/react-query-devtools'
import {
  createRootRoute,
  Outlet,
  useLocation,
  useNavigate
} from '@tanstack/react-router'
import { TanStackRouterDevtools } from '@tanstack/router-devtools'
import { useEffect } from 'react'

const AllowedRoutes = ['/auth/login', '/']

export const Route = createRootRoute({
  component: Root
})

function Root() {
  const { user, isUserLoading } = useUser()
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
      navigation({
        to: '/error'
      })
    } else if (!isError && location.pathname === '/error') {
      navigation({
        to: '/'
      })
    }
  }, [location, isError, navigation])

  // useEffect(() => {
  //   if (isError || isUserLoading) return
  //   if (!user) {
  //     if (!AllowedRoutes.includes(location.pathname)) {
  //       navigation({
  //         to: '/auth/login'
  //       })
  //     }
  //   }
  // }, [user, isUserLoading, location, isError, navigation])

  return (
    <>
      <div className="w-screen min-h-dvh">
        <Outlet />
      </div>
      <TanStackRouterDevtools />
      <ReactQueryDevtools initialIsOpen={false} />
      <Toaster />
    </>
  )
}
