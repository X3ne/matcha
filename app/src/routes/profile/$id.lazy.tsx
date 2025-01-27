import api from '@/api'
import { ProfileTag } from '@/api/spec'
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
import { useToast } from '@/components/ui/use-toast'
import { useUser } from '@/hooks/useUser'
import { useQuery, useMutation } from '@tanstack/react-query'
import {
  createLazyFileRoute,
  useParams,
  useNavigate
} from '@tanstack/react-router'
import { Heart, Eye, MapPin, Flame, AlertTriangle, Ban } from 'lucide-react'
import { useState, useEffect } from 'react'

export const Route = createLazyFileRoute('/profile/$id')({
  component: Profile
})

function Profile() {
  const navigate = useNavigate()
  const { userProfile: currentUserProfile } = useUser()
  const { id: userId } = useParams({ from: '/profile/$id' })

  useEffect(() => {
    if (currentUserProfile && currentUserProfile.id === userId) {
      navigate({ to: '/profile' })
    }
  }, [currentUserProfile, userId, navigate])

  const { data: userProfile, refetch: refetchUserProfile } = useQuery({
    queryKey: ['user', userId],
    retry: false,
    queryFn: async () =>
      (
        await api.v1.getUserProfileById(userId, {
          credentials: 'include'
        })
      ).data
  })

  const { toast } = useToast()

  const [isConfettiActive, setIsConfettiActive] = useState(false)

  const likeMutation = useMutation({
    mutationFn: async () => {
      return api.v1.likeUserProfile(userId)
    },
    onSuccess: () => {
      setIsConfettiActive(true)
      setTimeout(() => {
        setIsConfettiActive(false)
      }, 1000)
      toast({
        title: 'Liked',
        description: `You have liked ${userProfile?.name}'s profile.`
      })
    },
    onError: (err: any) => {
      console.error(err)
      toast({
        title: 'Action failed',
        description:
          err.message || 'Something went wrong while liking the profile.',
        variant: 'destructive'
      })
    },
    onSettled: () => {
      refetchUserProfile()
    }
  })

  const unlikeMutation = useMutation({
    mutationFn: async () => {
      return api.v1.removeUserProfileLike(userId)
    },
    onSuccess: () => {
      toast({
        title: 'Unliked',
        description: `You have unliked ${userProfile?.name}'s profile.`
      })
    },
    onError: (err: any) => {
      console.error(err)
      toast({
        title: 'Action failed',
        description:
          err.message || 'Something went wrong while unliking the profile.',
        variant: 'destructive'
      })
    },
    onSettled: () => {
      refetchUserProfile()
    }
  })

  const handleLike = () => {
    if (userProfile?.meta?.is_liked) {
      unlikeMutation.mutate()
    } else {
      likeMutation.mutate()
    }
  }

  const handleReport = () => {
    toast({
      title: 'Report Submitted',
      description: `You have reported ${userProfile?.name}'s profile as a fake account.`
    })
  }

  const handleBlock = () => {
    toast({
      title: 'User Blocked',
      description: `You have blocked ${userProfile?.name}. They will no longer appear in your search results or generate notifications.`
    })
  }

  return (
    <div className="flex flex-col items-center justify-center p-4">
      <Card className="w-full max-w-md border-0 shadow-none sm:border sm:shadow md:max-w-xl">
        <CardContent className="space-y-8 p-6">
          <div className="flex flex-col items-center md:flex-row md:items-start md:justify-between md:space-x-4">
            <div className="flex flex-col items-center md:flex-row md:items-center md:space-x-4">
              <Avatar className="h-20 w-20 border border-gray-300">
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
              <div className="text-center md:text-left">
                <h2 className="text-xl font-medium">
                  {userProfile?.name}, {userProfile?.age}
                </h2>
                <p className="text-[10px] text-gray-500">
                  {/* Last seen: {new Date(userProfile?.lastOnline).toLocaleString()} */}
                  Last seen: 2 hours ago
                </p>
                <div className="mt-2">
                  {/* {!userProfile?.isOnline && (
                    <span className="text-[0.65rem] text-gray-500">
                      Last seen:{' '}
                      {new Date(userProfile?.lastOnline).toLocaleString()}
                    </span>
                  )} */}
                </div>
              </div>
            </div>
            <div className="flex flex-col items-center space-y-2 pt-6 sm:pt-0 md:items-start">
              <Button
                size={'sm'}
                variant={userProfile?.meta?.is_liked ? 'outline' : 'default'}
                onClick={handleLike}
                className={`w-full ${
                  isConfettiActive
                    ? 'motion-preset-confetti motion-duration-700'
                    : ''
                }`}
              >
                <Heart className="h-4 w-4" />
                {userProfile?.meta?.is_liked ? 'Unlike' : 'Like'}
              </Button>
              {/* Report */}
              <Dialog>
                <DialogTrigger asChild>
                  <Button size={'sm'} variant="outline" className="w-full">
                    <AlertTriangle className="h-4 w-4" /> Report
                  </Button>
                </DialogTrigger>
                <DialogContent>
                  <DialogHeader>
                    <DialogTitle>Report Fake Account</DialogTitle>
                    <DialogDescription>
                      Are you sure you want to report this account as fake?
                    </DialogDescription>
                  </DialogHeader>
                  <div className="flex justify-end space-x-2">
                    <Button size={'sm'} variant="outline" onClick={() => {}}>
                      Cancel
                    </Button>
                    <Button onClick={handleReport}>Report</Button>
                  </div>
                </DialogContent>
              </Dialog>
              {/* Block */}
              <Dialog>
                <DialogTrigger asChild>
                  <Button size={'sm'} className="w-full" variant="outline">
                    <Ban className="h-4 w-4" /> Block
                  </Button>
                </DialogTrigger>
                <DialogContent>
                  <DialogHeader>
                    <DialogTitle>Block User</DialogTitle>
                    <DialogDescription>
                      Are you sure you want to block this user? They will no
                      longer appear in your search results or generate
                      notifications.
                    </DialogDescription>
                  </DialogHeader>
                  <div className="flex justify-end space-x-2">
                    <Button variant="outline" onClick={() => {}}>
                      Cancel
                    </Button>
                    <Button onClick={handleBlock}>Block</Button>
                  </div>
                </DialogContent>
              </Dialog>
              {/* {userProfile?.isConnected && (
                <Badge variant="default">Connected</Badge>
              )}
              {userProfile?.hasLikedCurrentUser &&
                !userProfile?.isConnected && (
                  <Badge variant="secondary">Liked your profile</Badge>
                )} */}
            </div>
          </div>

          <div className="flex flex-wrap justify-around gap-2 text-center text-xs text-gray-600">
            <div>
              <MapPin className="mr-1 inline-block h-4 w-4 text-primary" />
              <span>{userProfile?.approx_distance_km} km away</span>
            </div>
            <div>
              <Eye className="mr-1 inline-block h-4 w-4 text-primary" />
              {/* <span>{userProfile?.profileViewCount} views</span> */}
              <span>166 views</span>
            </div>
            <div>
              <Heart className="mr-1 inline-block h-4 w-4 text-primary" />
              {/* <span>{userProfile?.likeCount} likes</span> */}
              <span>22 likes</span>
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
                  {tag.name}
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
