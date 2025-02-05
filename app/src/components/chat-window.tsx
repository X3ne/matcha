import api, { type Message, MessageSortBy, SortOrder } from '@/api'
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { ScrollArea } from '@/components/ui/scroll-area'
import { useUser } from '@/hooks/useUser'
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'
import { Link } from '@tanstack/react-router'
import { SendHorizontal } from 'lucide-react'
import { useEffect, useState, useRef } from 'react'

interface ChatWindowProps {
  channelId: string
}

export function ChatWindow({ channelId }: ChatWindowProps) {
  const queryClient = useQueryClient()
  const { user, userProfile } = useUser()

  const {
    data: messages,
    isPending: messagesLoading,
    isError: messagesError
  } = useQuery({
    queryKey: ['channelMessages', channelId],
    queryFn: async () =>
      (
        await api.v1.getChannelMessages(channelId, {
          limit: 500,
          offset: 0,
          sort_by: MessageSortBy.SentAt,
          sort_order: SortOrder.Asc
        })
      ).data as Message[],
    staleTime: 0,
    enabled: !!channelId
  })

  const { data: channels } = useQuery({
    queryKey: ['myChannels'],
    queryFn: async () => (await api.v1.getMyChannels()).data
  })
  const channel = channels?.find((c) => c.id === channelId)
  const participant = channel?.participants?.filter(
    (participant) => participant.name !== userProfile?.name
  )[0]
  const avatarUrl = api.baseUrl + participant?.avatar

  const [newMessage, setNewMessage] = useState('')

  const [localMessages, setLocalMessages] = useState<Message[]>([])

  useEffect(() => {
    if (messages) {
      setLocalMessages(messages)
    }
  }, [messages])

  const { mutate: sendMessageMutation, isPending: isSending } = useMutation({
    mutationFn: async (content: string) => {
      await api.v1.postChannelMessage(channelId, {
        content
      })
    },
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: ['channelMessages', channelId],
        exact: true
      })
      queryClient.invalidateQueries({
        queryKey: ['lastMessage', channelId],
        exact: true
      })
    },
    onError: (error) => {
      console.error(error)
    }
  })

  const scrollAreaRef = useRef<HTMLDivElement | null>(null)

  useEffect(() => {
    if (scrollAreaRef.current) {
      const viewport = scrollAreaRef.current.querySelector(
        '[data-radix-scroll-area-viewport]'
      ) as HTMLDivElement | null

      if (viewport) {
        viewport.scrollTop = viewport.scrollHeight
      }
    }
  }, [localMessages])

  const handleSendMessage = () => {
    if (!newMessage.trim()) return

    const tempId = Date.now().toString()
    const now = new Date().toISOString().replace('Z', '000')

    setLocalMessages((prev) => [
      ...prev,
      {
        id: tempId,
        content: newMessage,
        sent_at: now,
        author: {
          id: userProfile?.id,
          name: user?.username,
          avatar: userProfile?.avatar_url
        }
      } as Message
    ])

    sendMessageMutation(newMessage)

    setNewMessage('')
  }

  if (!channelId) {
    return (
      <div className="flex h-full items-center justify-center">
        <p className="text-muted-foreground">No channel selected</p>
      </div>
    )
  }

  if (messagesLoading) {
    return (
      <div className="flex h-full items-center justify-center">
        <p>Loading messages...</p>
      </div>
    )
  }

  if (messagesError || !messages) {
    return (
      <div className="flex h-full items-center justify-center">
        <p className="text-red-500">Failed to load messages</p>
      </div>
    )
  }

  return (
    <div className="flex h-full w-full flex-col">
      <div className="border-b p-4">
        <div className="flex items-center gap-3">
          <Link to="/profile/$id" params={{ id: channel?.name.split('-')[1] }}>
            <Avatar className="h-10 w-10">
              {participant?.avatar ? (
                <AvatarImage
                  src={avatarUrl}
                  alt={participant?.name || 'Channel'}
                />
              ) : (
                <AvatarFallback>{(participant?.name || 'C')[0]}</AvatarFallback>
              )}
            </Avatar>
          </Link>
          <p className="truncate text-[18px] font-medium">
            {participant?.name.split(' ')[0]}
          </p>
        </div>
      </div>

      <ScrollArea ref={scrollAreaRef} className="h-[calc(100vh-21rem)]">
        <div className="space-y-4 px-4 pt-4">
          {localMessages.map((message: Message) => {
            const isCurrentUser = message.author?.id === userProfile?.id

            return (
              <div
                key={message.id}
                className={`flex ${
                  isCurrentUser ? 'justify-end' : 'justify-start'
                }`}
              >
                <div className="flex max-w-[26rem] flex-col">
                  <div
                    className={`w-fit max-w-[26rem] break-words rounded-lg p-3 ${
                      isCurrentUser
                        ? 'ml-auto bg-primary text-primary-foreground'
                        : 'mr-auto bg-muted'
                    }`}
                  >
                    <p className="w-full text-xs">{message.content}</p>
                  </div>
                  <p
                    className={`mt-1 w-fit text-[8px] opacity-70 ${
                      isCurrentUser
                        ? 'ml-auto -translate-x-1'
                        : 'mr-auto translate-x-1'
                    }`}
                  >
                    {new Date(message.sent_at + 'Z').toLocaleTimeString([], {
                      hour: '2-digit',
                      minute: '2-digit'
                    })}
                  </p>
                </div>
              </div>
            )
          })}
        </div>
      </ScrollArea>

      <div className="mt-auto border-t px-4 py-3">
        <div className="flex gap-2">
          <Input
            placeholder="Type a message..."
            className="border-none bg-gray-100 shadow-none placeholder:text-xs placeholder:text-gray-700"
            value={newMessage}
            onChange={(e) => setNewMessage(e.target.value)}
            onKeyDown={(e) => {
              if (e.key === 'Enter') {
                handleSendMessage()
              }
            }}
            disabled={isSending}
          />
          <Button onClick={handleSendMessage} size="icon" disabled={isSending}>
            <SendHorizontal className="h-4 w-4" />
          </Button>
        </div>
      </div>
    </div>
  )
}
