import { ProfileTag } from '@/api/spec'
import { EditProfileForm } from '@/components/edit-profile-form'
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar'
import { Button } from '@/components/ui/button'
import { Card, CardContent } from '@/components/ui/card'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
  DialogTrigger
} from '@/components/ui/dialog'
import { ScrollArea, ScrollBar } from '@/components/ui/scroll-area'
import { UserContext } from '@/providers/userProvider'
import { createLazyFileRoute } from '@tanstack/react-router'
import {
  Heart,
  Mail,
  Flame,
  Edit,
  Activity,
  Eye,
  MessageSquare,
  ThumbsUp,
  ThumbsDown
} from 'lucide-react'
import { useContext, useState } from 'react'

export const Route = createLazyFileRoute('/profile/')({
  component: Profile
})

function Profile() {
  const { user, userProfile } = useContext(UserContext)
  const [isActivityOpen, setIsActivityOpen] = useState(false)
  const [isEditProfileOpen, setEditProfileOpen] = useState(false)

  type Notification = {
    id: string
    type: 'like' | 'view' | 'message' | 'match' | 'unlike'
    content: string
    timestamp: string
  }

  const mockNotifications: Notification[] = [
    {
      id: '1',
      type: 'like',
      content: 'John Doe liked your profile',
      timestamp: '2023-04-20T10:30:00Z'
    },
    {
      id: '2',
      type: 'view',
      content: 'Emma Doe viewed your profile',
      timestamp: '2023-04-19T15:45:00Z'
    },
    {
      id: '3',
      type: 'message',
      content: 'You have a new message from Sarah',
      timestamp: '2023-04-18T09:15:00Z'
    },
    {
      id: '4',
      type: 'match',
      content: 'You and Emily are a match!',
      timestamp: '2023-04-17T14:20:00Z'
    },
    {
      id: '5',
      type: 'unlike',
      content: 'Mike unliked your profile',
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

  return (
    <div className="flex flex-col items-center justify-center p-4">
      <Card className="w-full max-w-md shadow-none sm:shadow md:max-w-xl">
        <CardContent className="space-y-8 p-6">
          <div className="flex flex-col md:flex-row md:items-start md:justify-between md:space-x-10">
            <div className="flex flex-col md:flex-row md:items-center md:space-x-4">
              <Avatar className="h-20 w-20 border">
                <AvatarImage
                  className="object-cover"
                  src={import.meta.env.VITE_API_URL + userProfile?.avatar_url}
                  alt={userProfile?.name ?? 'Profile picture'}
                />
                <AvatarFallback>
                  {userProfile?.name
                    ?.split(' ')
                    .map((n) => n[0])
                    .join('')}
                </AvatarFallback>
              </Avatar>
              <div>
                <h2 className="text-xl font-medium">
                  {userProfile?.name}, {userProfile?.age}
                </h2>
                <p className="text-[11px] text-gray-500">
                  {/* {userProfile?.gender} • {userProfile?.sexual_orientation} */}
                  @{user?.username}
                </p>
              </div>
            </div>

            <div className="flex flex-col items-center space-y-2 pt-6 sm:pt-0 md:items-start">
              <Dialog
                open={isEditProfileOpen}
                onOpenChange={setEditProfileOpen}
              >
                <DialogTrigger asChild>
                  <Button variant="outline" size="sm" className="w-full">
                    <Edit className="h-4 w-4" /> Edit Profile
                  </Button>
                </DialogTrigger>
                <DialogContent className="!p-0 sm:max-w-3xl">
                  <DialogHeader className="p-6 pb-0">
                    <DialogTitle>Edit Profile</DialogTitle>
                    <DialogDescription className="text-xs">
                      Update your profile information
                    </DialogDescription>
                  </DialogHeader>
                  <ScrollArea className="mt-4 max-h-[60vh] px-6 pb-6">
                    <div className="">
                      <EditProfileForm />
                    </div>
                  </ScrollArea>
                </DialogContent>
              </Dialog>
              <Dialog open={isActivityOpen} onOpenChange={setIsActivityOpen}>
                <DialogTrigger asChild>
                  <Button variant="outline" size="sm" className="w-full">
                    <Activity className="h-4 w-4" /> Activity
                  </Button>
                </DialogTrigger>
                <DialogContent className="sm:max-w-[425px]">
                  <DialogHeader>
                    <DialogTitle>Profile Activity</DialogTitle>
                    <DialogDescription className="text-xs">
                      Recent profile views and likes
                    </DialogDescription>
                  </DialogHeader>
                  <ScrollArea className="mt-4 max-h-[80vh]">
                    {mockNotifications.length === 0 ? (
                      <p className="py-4 text-center text-gray-500">
                        No notifications
                      </p>
                    ) : (
                      mockNotifications.map((notification) => (
                        <div
                          key={notification.id}
                          className={'flex items-start py-3'}
                        >
                          <div className="mr-3 flex-shrink-0">
                            <NotificationIcon type={notification.type} />
                          </div>
                          <div className="flex-grow">
                            <p className="text-sm">{notification.content}</p>
                            <p className="mt-1 text-xs text-gray-500">
                              {new Date(
                                notification.timestamp
                              ).toLocaleString()}
                            </p>
                          </div>
                        </div>
                      ))
                    )}
                  </ScrollArea>
                </DialogContent>
              </Dialog>
            </div>
          </div>

          <div className="flex flex-wrap justify-around gap-2 text-center text-xs text-gray-600">
            <div>
              <Mail className="mr-1 inline-block h-4 w-4 text-primary" />
              <span>{user?.email}</span>
            </div>
            <div>
              <Eye className="mr-1 inline-block h-4 w-4 text-primary" />
              {/* <span>{userProfile?.profileViewCount} views</span> */}
              <span>166 views</span>
            </div>
            <div>
              <Heart className="mr-1 inline-block h-4 w-4 text-primary" />
              {/* <span>{userProfile?.likeCount} likes</span> */}
              <span>20 likes</span>
            </div>
            <div>
              <Flame className="mr-1 inline-block h-4 w-4 text-primary" />
              <span>{userProfile?.rating} fame</span>
            </div>
          </div>

          <div className="text-center md:text-left">
            <h3 className="mb-2 font-medium">About Me</h3>
            <p className="text-sm text-gray-600">{userProfile?.bio}</p>
          </div>

          <div className="text-center md:text-left">
            <h3 className="mb-2 font-medium">Interests</h3>
            <div className="flex flex-wrap justify-center gap-2 sm:justify-start">
              {userProfile?.tags?.map((tag: ProfileTag) => (
                <div
                  key={tag.id}
                  className="inline-flex items-center rounded-full border border-white/20 bg-black/80 px-2.5 py-1 text-[8px] font-normal text-white"
                >
                  {tag.name.split('_')[0]}
                </div>
              ))}
            </div>
          </div>

          <div className="text-center md:text-left">
            <h3 className="mb-2 font-medium">Photos</h3>
            <ScrollArea className="">
              <div className="flex gap-2 pb-3">
                {userProfile?.picture_urls.map((imgUrl, i) => (
                  <img
                    key={i}
                    className="h-32 w-32 rounded-md object-cover"
                    src={import.meta.env.VITE_API_URL + imgUrl}
                    alt={`Additional photo ${i + 1}`}
                  />
                ))}
              </div>
              <ScrollBar orientation="horizontal" />
            </ScrollArea>
          </div>
        </CardContent>
      </Card>
    </div>
  )
}
