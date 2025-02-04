import api from '@/api'
import React, { createContext, useEffect, useState } from 'react'

type GatewayEventOp =
  | 'MessageReceived'
  | 'LikeReceived'
  | 'ProfileViewed'
  | 'NewMatch'
  | 'MatchRemoved'
  | 'SystemNotification'

interface GatewayEvent {
  op: GatewayEventOp
  data: any
}

interface GatewayContextValue {
  events: GatewayEvent[] | undefined
}

export const GatewayContext = createContext<GatewayContextValue | undefined>({
  events: undefined
})

export function GatewayProvider({ children }: { children: React.ReactNode }) {
  const [events, setEvents] = useState<GatewayEvent[]>([])

  useEffect(() => {
    const eventSource = new EventSource(api.baseUrl + '/v1/gateway', {
      withCredentials: true
    })

    eventSource.onmessage = (event) => {
      try {
        const parsed: GatewayEvent = JSON.parse(event.data)
        setEvents((prev) => [...prev, parsed])
      } catch (err) {
        console.error('Failed to parse SSE event:', err)
      }
    }

    eventSource.onerror = (err) => {
      console.error('SSE error:', err)
    }

    return () => {
      eventSource.close()
    }
  }, [])

  return (
    <GatewayContext.Provider value={{ events }}>
      {children}
    </GatewayContext.Provider>
  )
}
