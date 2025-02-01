import { MessagesLayout } from '@/components/message-layout'
import { createLazyFileRoute } from '@tanstack/react-router'

export const Route = createLazyFileRoute('/messages/')({
  component: MessagesIndex
})

function MessagesIndex() {
  return (
    <MessagesLayout>
      <div className="flex h-full items-center justify-center">
        <p className="text-muted-foreground">
          Please select a conversation on the left
        </p>
      </div>
    </MessagesLayout>
  )
}
