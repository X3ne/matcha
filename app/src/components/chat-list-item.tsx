import api, {
  type Channel,
  type Message,
  MessageSortBy,
  SortOrder
} from '@/api'
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar'
import { useUser } from '@/hooks/useUser'
import { cn } from '@/lib/utils'
import { useQuery } from '@tanstack/react-query'
import { Link } from '@tanstack/react-router'

export function ChatListItem({ channel }: { channel: Channel }) {
  const { userProfile } = useUser()
  const participant = channel.participants?.filter(
    (participant) => participant.name !== userProfile?.name
  )[0]
  const avatarUrl = api.baseUrl + participant?.avatar

  const { data: messages } = useQuery({
    queryKey: ['channelMessages', channel.id],
    queryFn: async () =>
      (
        await api.v1.getChannelMessages(channel.id, {
          limit: 1,
          offset: 0,
          sort_by: MessageSortBy.SentAt,
          sort_order: SortOrder.Asc
        })
      ).data as Message[],
    enabled: !!channel.id
  })

  return (
    <Link
      to="/messages/$id"
      params={{ id: channel.id }}
      className={cn(
        'flex cursor-pointer items-center gap-3 border-b p-3 transition-colors hover:bg-accent'
      )}
      activeProps={{
        'data-status': 'active'
      }}
    >
      <Avatar className="h-10 w-10">
        {avatarUrl ? (
          <AvatarImage src={avatarUrl} alt={participant?.name || 'Channel'} />
        ) : (
          <AvatarFallback>{(participant?.name || 'C')[0]}</AvatarFallback>
        )}
      </Avatar>

      <div className="min-w-0 flex-1">
        <p className="truncate font-medium">
          {participant?.name.split(' ')[0]}
        </p>
        <p className="truncate text-xs text-muted-foreground">
          {messages?.[0]?.content || 'No recent messages'}
        </p>
      </div>

      <span className="whitespace-nowrap text-xs text-muted-foreground">
        {messages?.[0]?.sent_at
          ? new Date(messages[0].sent_at).toLocaleTimeString([], {
              hour: '2-digit',
              minute: '2-digit'
            })
          : '--'}
      </span>
    </Link>
  )
}
