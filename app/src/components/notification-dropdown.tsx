'use client'

import { Button } from '@/components/ui/button'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuTrigger
} from '@/components/ui/dropdown-menu'
import { ScrollArea } from '@/components/ui/scroll-area'
import { useGatewayEvents } from '@/hooks/useGatewayEvents'
import {
  Bell,
  Heart,
  Eye,
  MessageSquare,
  ThumbsUp,
  ThumbsDown
} from 'lucide-react'
import { useEffect, useState } from 'react'

type Notification = {
  id: string
  type: 'like' | 'view' | 'message' | 'match' | 'unlike' | 'system'
  content: string
  read: boolean
  timestamp: string
}

function generateUniqueId(): string {
  return crypto.randomUUID ? crypto.randomUUID() : String(Date.now())
}

const NotificationIcon = ({ type }: { type: Notification['type'] }) => {
  switch (type) {
    case 'like':
      return <Heart className="h-4 w-4 text-red-500" />
    case 'view':
      return <Eye className="h-4 w-4 text-blue-500" />
    case 'message':
      return <MessageSquare className="h-4 w-4 text-green-500" />
    case 'match':
      return <ThumbsUp className="h-4 w-4 text-purple-500" />
    case 'unlike':
      return <ThumbsDown className="h-4 w-4 text-yellow-500" />
    case 'system':
      return <Bell className="h-4 w-4 text-gray-500" />
  }
}

export function NotificationDropdown() {
  const [notifications, setNotifications] = useState<Notification[]>([])

  const { events } = useGatewayEvents()

  useEffect(() => {
    console.log('SSE events:', events)
    if (events.length === 0) return

    const latestEvent = events[events.length - 1]

    let newItem: Notification | null = null
    const now = new Date().toISOString()

    switch (latestEvent.op) {
      case 'LikeReceived':
        newItem = {
          id: generateUniqueId(),
          type: 'like',
          content: `${latestEvent.data.username} liked your profile`,
          read: false,
          timestamp: now
        }
        break

      case 'ProfileViewed':
        newItem = {
          id: generateUniqueId(),
          type: 'view',
          content: `${latestEvent.data.username} viewed your profile`,
          read: false,
          timestamp: now
        }
        break

      case 'MessageReceived':
        newItem = {
          id: generateUniqueId(),
          type: 'message',
          content: `New message from ${latestEvent.data.sender_username}`,
          read: false,
          timestamp: now
        }
        break

      case 'NewMatch':
        newItem = {
          id: generateUniqueId(),
          type: 'match',
          content: `You and ${latestEvent.data.username} are a match!`,
          read: false,
          timestamp: now
        }
        break

      case 'MatchRemoved':
        newItem = {
          id: generateUniqueId(),
          type: 'unlike',
          content: `${latestEvent.data.username} removed the match`,
          read: false,
          timestamp: now
        }
        break

      case 'SystemNotification':
        newItem = {
          id: generateUniqueId(),
          type: 'system',
          content: latestEvent.data.message,
          read: false,
          timestamp: now
        }
        break

      default:
        // Unknown event: skip or handle
        console.warn('Unhandled SSE event:', latestEvent)
        break
    }

    if (newItem) {
      setNotifications((prev) => [newItem!, ...prev])
    }
  }, [events])

  const unreadCount = notifications.filter((n) => !n.read).length

  const markAsRead = (id: string) => {
    setNotifications((prev) =>
      prev.map((n) => (n.id === id ? { ...n, read: true } : n))
    )
  }

  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <Button variant="ghost" size="icon" className="relative">
          <Bell className="!size-5" />
          {unreadCount > 0 && (
            <span className="absolute right-0.5 top-0 flex h-3 w-3 items-center justify-center rounded-full bg-red-500 text-[8px] text-white">
              {unreadCount}
            </span>
          )}
        </Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent className="w-80" align="end">
        <ScrollArea className="h-[300px]">
          {notifications.length === 0 ? (
            <p className="py-4 text-center text-gray-500">No notifications</p>
          ) : (
            notifications.map((notification) => (
              <div
                key={notification.id}
                className={`flex items-start p-3 hover:bg-gray-100 ${
                  notification.read ? 'opacity-50' : ''
                }`}
                onClick={() => markAsRead(notification.id)}
              >
                <div className="mr-3 flex-shrink-0">
                  <NotificationIcon type={notification.type} />
                </div>
                <div className="flex-grow">
                  <p className="text-sm">{notification.content}</p>
                  <p className="mt-1 text-xs text-gray-500">
                    {new Date(notification.timestamp).toLocaleString()}
                  </p>
                </div>
                {!notification.read && (
                  <div className="ml-2 mt-1 h-2 w-2 rounded-full bg-blue-500" />
                )}
              </div>
            ))
          )}
        </ScrollArea>
      </DropdownMenuContent>
    </DropdownMenu>
  )
}
