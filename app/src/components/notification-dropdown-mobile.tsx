'use client'

import { Button } from '@/components/ui/button'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuTrigger
} from '@/components/ui/dropdown-menu'
import { ScrollArea } from '@/components/ui/scroll-area'
import {
  Bell,
  Heart,
  Eye,
  MessageSquare,
  ThumbsUp,
  ThumbsDown
} from 'lucide-react'
import { useState } from 'react'

type Notification = {
  id: string
  type: 'like' | 'view' | 'message' | 'match' | 'unlike'
  content: string
  read: boolean
  timestamp: string
}

const mockNotifications: Notification[] = [
  {
    id: '1',
    type: 'like',
    content: 'John Doe liked your profile',
    read: false,
    timestamp: '2023-04-20T10:30:00Z'
  },
  {
    id: '2',
    type: 'view',
    content: 'Emma Doe viewed your profile',
    read: false,
    timestamp: '2023-04-19T15:45:00Z'
  },
  {
    id: '3',
    type: 'message',
    content: 'You have a new message from Sarah',
    read: true,
    timestamp: '2023-04-18T09:15:00Z'
  },
  {
    id: '4',
    type: 'match',
    content: 'You and Emily are a match!',
    read: false,
    timestamp: '2023-04-17T14:20:00Z'
  },
  {
    id: '5',
    type: 'unlike',
    content: 'Mike unliked your profile',
    read: true,
    timestamp: '2023-04-16T11:10:00Z'
  }
]

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
  }
}

export function NotificationDropdownMobile() {
  const [notifications, setNotifications] =
    useState<Notification[]>(mockNotifications)

  const unreadCount = notifications.filter((n) => !n.read).length

  const markAsRead = (id: string) => {
    setNotifications(
      notifications.map((n) => (n.id === id ? { ...n, read: true } : n))
    )
  }

  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <Button variant="outline">
          Notifications
          {unreadCount > 0 && (
            <span className="relative h-4 w-4 rounded-full bg-red-500">
              <span className="absolute -top-0.5 right-[5px] text-[9px] text-white">
                {unreadCount}
              </span>
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
                className={`flex items-start p-3 hover:bg-gray-100 ${notification.read ? 'opacity-50' : ''}`}
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
