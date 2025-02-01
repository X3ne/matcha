import api from '@/api'
import { ChatListItem } from '@/components/chat-list-item'
import { ScrollArea } from '@/components/ui/scroll-area'
import { useQuery } from '@tanstack/react-query'

export function ChatList() {
  const {
    data: channels,
    isLoading,
    isError
  } = useQuery({
    queryKey: ['myChannels'],
    queryFn: async () => (await api.v1.getMyChannels()).data
  })

  if (isLoading) {
    return (
      <div className="p-4">
        <p>Loading channels...</p>
      </div>
    )
  }

  if (isError || !channels) {
    return (
      <div className="p-4">
        <p>Failed to load channels</p>
      </div>
    )
  }

  return (
    <div className="flex h-full min-w-min flex-col sm:min-w-[18rem]">
      <h2 className="border-b p-4 text-2xl font-medium">Messages</h2>
      <ScrollArea className="h-full flex-1">
        {channels.map((channel) => (
          <ChatListItem key={channel.id} channel={channel} />
        ))}
      </ScrollArea>
    </div>
  )
}
