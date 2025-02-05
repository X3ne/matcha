import { ChatList } from '@/components/chat-list'
import React from 'react'

export function MessagesLayout({ children }: { children?: React.ReactNode }) {
  return (
    <div className="flex h-full gap-4">
      <div className="rounded-lg bg-white shadow sm:flex">
        <ChatList />
      </div>

      <div className="flex-1 rounded-lg bg-white shadow">{children}</div>
    </div>
  )
}
