import { GatewayContext } from '@/providers/gatewayEventsProvider'
import { useContext } from 'react'

export const useGatewayEvents = () => {
  const context = useContext(GatewayContext)

  if (!context) {
    throw new Error('useGatewayEvents must be used within a GatewayProvider')
  }

  return context
}
