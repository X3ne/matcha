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
import { useToast } from '@/components/ui/use-toast'
import { useUser } from '@/hooks/useUser'
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

  const userProfile = {
    id: '123',
    name: 'Alex Johnson',
    username: 'alexj',
    age: 25,
    gender: 'Non-binary',
    sexualPreferences: 'Pansexual',
    bio: 'Adventurous soul with a passion for technology and nature. Always looking for the next big challenge!',
    interests: ['vegan', 'geek', 'piercing', 'hiking', 'photography'],
    profilePicture: '/placeholder.svg?height=100&width=100',
    additionalPhotos: [
      '/placeholder.svg?height=80&width=80',
      '/placeholder.svg?height=80&width=80',
      '/placeholder.svg?height=80&width=80',
      '/placeholder.svg?height=80&width=80'
    ],
    profileViewCount: 1234,
    likeCount: 567,
    fameRating: 4.5,
    location: 'Downtown, Metropolis',
    isOnline: false,
    lastOnline: '2023-04-15T14:30:00Z',
    isConnected: false,
    hasLikedCurrentUser: false,
    isLikedByCurrentUser: false
  }

  const [profile, setProfile] = useState(userProfile)
  const { toast } = useToast()

  const [isConfettiActive, setIsConfettiActive] = useState(false)

  const handleLike = () => {
    if (!profile.isLikedByCurrentUser) {
      setIsConfettiActive(true)
      setTimeout(() => {
        setIsConfettiActive(false)
      }, 1000)
    }

    setProfile((prev) => ({
      ...prev,
      isLikedByCurrentUser: !prev.isLikedByCurrentUser
    }))
    toast({
      title: profile.isLikedByCurrentUser ? 'Unliked' : 'Liked',
      description: `You have ${profile.isLikedByCurrentUser ? 'unliked' : 'liked'} ${profile.name}'s profile.`
    })
  }

  const handleReport = () => {
    toast({
      title: 'Report Submitted',
      description: `You have reported ${profile.name}'s profile as a fake account.`
    })
  }

  const handleBlock = () => {
    toast({
      title: 'User Blocked',
      description: `You have blocked ${profile.name}. They will no longer appear in your search results or generate notifications.`
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
                  alt={profile.name}
                />
                <AvatarFallback>
                  {profile.name
                    .split(' ')
                    .map((n) => n[0])
                    .join('')}
                </AvatarFallback>
              </Avatar>
              <div className="text-center md:text-left">
                <h2 className="text-xl font-semibold">
                  {profile.name}, {profile.age}
                </h2>
                <p className="text-xs text-gray-500">
                  {profile.gender} • {profile.sexualPreferences}
                </p>
                <div className="mt-2">
                  {!profile.isOnline && (
                    <span className="text-[0.65rem] text-gray-500">
                      Last seen: {new Date(profile.lastOnline).toLocaleString()}
                    </span>
                  )}
                </div>
              </div>
            </div>
            <div className="flex flex-col items-center space-y-2 pt-6 sm:pt-0 md:items-start">
              <Button
                size={'sm'}
                variant={profile.isLikedByCurrentUser ? 'default' : 'outline'}
                onClick={handleLike}
                className={`w-full ${
                  isConfettiActive
                    ? 'motion-preset-confetti motion-duration-700'
                    : ''
                }`}
              >
                <Heart className="h-4 w-4" />
                {profile.isLikedByCurrentUser ? 'Unlike' : 'Like'}
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
              {profile.isConnected && (
                <Badge variant="default">Connected</Badge>
              )}
              {profile.hasLikedCurrentUser && !profile.isConnected && (
                <Badge variant="secondary">Liked your profile</Badge>
              )}
            </div>
          </div>

          <div className="flex flex-wrap justify-around gap-2 text-center text-xs text-gray-500">
            <div>
              <MapPin className="mr-1 inline-block h-4 w-4" />
              <span>{profile.location}</span>
            </div>
            <div>
              <Eye className="mr-1 inline-block h-4 w-4" />
              <span>{profile.profileViewCount} views</span>
            </div>
            <div>
              <Heart className="mr-1 inline-block h-4 w-4" />
              <span>{profile.likeCount} likes</span>
            </div>
            <div>
              <Flame className="mr-1 inline-block h-4 w-4" />
              <span>{profile.fameRating} fame</span>
            </div>
          </div>

          <div className="text-center md:text-left">
            <h3 className="mb-2 font-semibold">About Me</h3>
            <p className="text-sm">{profile.bio}</p>
          </div>

          <div className="text-center md:text-left">
            <h3 className="mb-2 font-semibold">Interests</h3>
            <div className="flex flex-wrap justify-center gap-2 sm:justify-start">
              {profile.interests.map((interest, index) => (
                <div
                  key={index}
                  className="inline-flex items-center rounded-full border border-white/20 bg-black/80 px-2.5 py-1 text-[8px] font-normal text-white"
                >
                  {interest}
                </div>
              ))}
            </div>
          </div>

          <div className="text-center md:text-left">
            <h3 className="mb-2 font-semibold">Photos</h3>
            <div className="flex space-x-2 overflow-x-auto pb-2">
              <img
                className="h-28 w-28 rounded-md object-cover"
                src="https://bonnierpublications.com/app/uploads/2022/05/woman-1-480x630.jpg"
                alt=""
              />
              <img
                className="h-28 w-28 rounded-md object-cover"
                src="https://bonnierpublications.com/app/uploads/2022/05/woman-1-480x630.jpg"
                alt=""
              />
              <img
                className="h-28 w-28 rounded-md object-cover"
                src="https://bonnierpublications.com/app/uploads/2022/05/woman-1-480x630.jpg"
                alt=""
              />
              <img
                className="h-28 w-28 rounded-md object-cover"
                src="https://bonnierpublications.com/app/uploads/2022/05/woman-1-480x630.jpg"
                alt=""
              />
              {/* Additional Images */}
            </div>
          </div>
        </CardContent>
      </Card>
    </div>
  )
}
