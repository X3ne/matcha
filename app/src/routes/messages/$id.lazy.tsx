import { ChatWindow } from '@/components/chat-window'
import { MessagesLayout } from '@/components/message-layout'
import { createLazyFileRoute } from '@tanstack/react-router'

export const Route = createLazyFileRoute('/messages/$id')({
  component: ChatWindowRoute
})

function ChatWindowRoute() {
  const { id } = Route.useParams()

  return (
    <MessagesLayout>
      <ChatWindow channelId={id} />
    </MessagesLayout>
  )
}
