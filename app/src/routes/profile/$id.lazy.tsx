import api from '@/api'
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar'
import { Badge } from '@/components/ui/badge'
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
import { useQuery } from '@tanstack/react-query'
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

  const { data: userProfile } = useQuery({
    queryKey: ['user', userId],
    retry: false,
    queryFn: async () =>
      (
        await api.v1.getUserProfileById(userId, {
          credentials: 'include'
        })
      ).data
  })

  // const [profile, setProfile] = useState(userProfile)
  const { toast } = useToast()

  const [isConfettiActive, setIsConfettiActive] = useState(false)

  const handleLike = () => {
    // if (!profile.isLikedByCurrentUser) {
    //   setIsConfettiActive(true)
    //   setTimeout(() => {
    //     setIsConfettiActive(false)
    //   }, 1000)
  }

  //   setProfile((prev) => ({
  //     ...prev,
  //     isLikedByCurrentUser: !prev.isLikedByCurrentUser
  //   }))
  //   toast({
  //     title: profile.isLikedByCurrentUser ? 'Unliked' : 'Liked',
  //     description: `You have ${profile.isLikedByCurrentUser ? 'unliked' : 'liked'} ${profile.name}'s profile.`
  //   })
  // }

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
      <Card className="w-full max-w-md border-0 shadow-none sm:border sm:shadow md:max-w-2xl">
        <CardContent className="space-y-8 p-6">
          <div className="flex flex-col items-center md:flex-row md:items-start md:justify-between md:space-x-4">
            <div className="flex flex-col items-center md:flex-row md:items-center md:space-x-4">
              <Avatar className="mx-auto h-20 w-20">
                <AvatarImage
                  className="object-cover"
                  src="https://bonnierpublications.com/app/uploads/2022/05/woman-1-480x630.jpg"
                  alt={userProfile?.name}
                />
                <AvatarFallback>
                  {/* {userProfile?.name
                    .split(' ')
                    .map((n) => n[0])
                    .join('')} */}
                </AvatarFallback>
              </Avatar>
              <div className="text-center md:text-left">
                <h2 className="text-xl font-semibold">
                  {userProfile?.name}, {userProfile?.age}
                </h2>
                <p className="text-xs text-gray-500">
                  {/* {userProfile?.gender} • {userProfile?.sexualPreferences} */}
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
                // variant={
                //   userProfile?.isLikedByCurrentUser ? 'default' : 'outline'
                // }
                onClick={handleLike}
                className={`w-full ${
                  isConfettiActive
                    ? 'motion-preset-confetti motion-duration-700'
                    : ''
                }`}
              >
                <Heart className="h-4 w-4" />
                {/* {userProfile?.isLikedByCurrentUser ? 'Unlike' : 'Like'} */}
              </Button>
              {/* Dialog for Report */}
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
              {/* Dialog for Block */}
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

          <div className="flex flex-wrap justify-around gap-2 text-center text-xs text-gray-500">
            <div>
              <MapPin className="mr-1 inline-block h-4 w-4" />
              <span>{userProfile?.approx_distance_km} away</span>
            </div>
            <div>
              <Eye className="mr-1 inline-block h-4 w-4" />
              {/* <span>{userProfile?.profileViewCount} views</span> */}
              <span>166 views</span>
            </div>
            <div>
              <Heart className="mr-1 inline-block h-4 w-4" />
              {/* <span>{userProfile?.likeCount} likes</span> */}
              <span>22 likes</span>
            </div>
            <div>
              <Flame className="mr-1 inline-block h-4 w-4" />
              {/* <span>{userProfile?.fameRating} fame</span> */}
              <span>10 fame</span>
            </div>
          </div>

          <div className="text-center md:text-left">
            <h3 className="mb-2 font-semibold">About Me</h3>
            <p className="text-sm">{userProfile?.bio}</p>
          </div>

          <div className="text-center md:text-left">
            <h3 className="mb-2 font-semibold">Interests</h3>
            <div className="flex flex-wrap justify-center gap-2 sm:justify-start">
              {/* {userProfile?.interests.map((interest, index) => (
                <div
                  key={index}
                  className="inline-flex items-center rounded-full border border-white/20 bg-black/80 px-2.5 py-1 text-[8px] font-normal text-white"
                >
                  {interest}
                </div>
              ))} */}
            </div>
          </div>

          <div className="text-center md:text-left">
            <h3 className="mb-2 font-semibold">Photos</h3>
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
