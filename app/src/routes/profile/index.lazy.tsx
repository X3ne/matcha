import { Tag } from '@/api/spec'
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
import { ScrollArea } from '@/components/ui/scroll-area'
import { UserContext } from '@/providers/userProvider'
import { createLazyFileRoute } from '@tanstack/react-router'
import { Heart, Mail, MapPin, Flame, Edit, Activity } from 'lucide-react'
import { useContext, useState } from 'react'

export const Route = createLazyFileRoute('/profile/')({
  component: Profile
})

function Profile() {
  const { user, userProfile } = useContext(UserContext)
  const [isActivityOpen, setIsActivityOpen] = useState(false)
  const [isEditProfileOpen, setEditProfileOpen] = useState(false)

  return (
    <div className="flex flex-col items-center justify-center p-4">
      <Card className="w-full max-w-md border-0 shadow-none sm:border sm:shadow md:max-w-2xl">
        <CardContent className="space-y-8 p-6">
          <div className="flex flex-col items-center md:flex-row md:items-start md:justify-between md:space-x-4">
            <div className="flex flex-col items-center md:flex-row md:items-center md:space-x-4">
              <Avatar className="h-20 w-20">
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
                <h2 className="text-xl font-semibold">
                  {userProfile?.name}, {userProfile?.age}
                </h2>
                <p className="text-xs text-gray-500">
                  {userProfile?.gender} • {userProfile?.sexual_orientation}
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
                <DialogContent className="sm:max-w-4xl">
                  <DialogHeader>
                    <DialogTitle>Edit Profile</DialogTitle>
                    <DialogDescription className="text-xs">
                      Update your profile information
                    </DialogDescription>
                  </DialogHeader>
                  <ScrollArea className="mt-4 max-h-[60vh]">
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
                  <ScrollArea className="mt-4 max-h-[60vh]">
                    <div className="space-y-4">
                      <div>
                        <h4 className="mb-2 text-sm font-semibold">
                          Profile Views
                        </h4>
                        {/* {userProfile?.views.map((view) => (
                          <div
                            key={view.id}
                            className="mb-2 flex items-center justify-between text-xs"
                          >
                            <span>{view.name}</span>
                            <span className="text-xs text-gray-500">
                              {new Date(view.date).toLocaleString()}
                            </span>
                          </div>
                        ))} */}
                      </div>
                      <div>
                        <h4 className="mb-2 text-sm font-semibold">Likes</h4>
                        {/* {userProfile?.likes.map((like) => (
                          <div
                            key={like.id}
                            className="mb-2 flex items-center justify-between text-xs"
                          >
                            <span>{like.name}</span>
                            <span className="text-xs text-gray-500">
                              {new Date(like.date).toLocaleString()}
                            </span>
                          </div>
                        ))} */}
                      </div>
                    </div>
                  </ScrollArea>
                </DialogContent>
              </Dialog>
            </div>
          </div>

          <div className="flex flex-wrap justify-around gap-2 text-center text-xs text-gray-500">
            <div>
              <MapPin className="mr-1 inline-block h-4 w-4" />
              {/* <span>{userProfile?.location}</span> */}
              <span>Lyon</span>
            </div>
            <div>
              <Mail className="mr-1 inline-block h-4 w-4" />
              <span>{user?.email}</span>
            </div>
            <div>
              <Heart className="mr-1 inline-block h-4 w-4" />
              {/* <span>{userProfile?.likeCount} likes</span> */}
              <span>20 likes</span>
            </div>
            <div>
              <Flame className="mr-1 inline-block h-4 w-4" />
              {/* <span>{userProfile?.fameRating} fame</span> */}
              <span>56 fame</span>
            </div>
          </div>

          <div className="text-center md:text-left">
            <h3 className="mb-2 font-semibold">About Me</h3>
            <p className="text-sm">{userProfile?.bio}</p>
          </div>

          <div className="text-center md:text-left">
            <h3 className="mb-2 font-semibold">Interests</h3>
            <div className="flex flex-wrap justify-center gap-2 sm:justify-start">
              {userProfile?.tags?.map((tag: Tag) => (
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
            <h3 className="mb-2 font-semibold">Photos</h3>
            <ScrollArea className="flex space-x-2 overflow-x-auto pb-2">
              {/* {userProfile?.images
                ?.slice(1)
                .map((imgUrl, i) => (
                  <img
                    key={i}
                    className="h-28 w-28 rounded-md object-cover"
                    src={imgUrl}
                    alt={`Additional photo ${i + 1}`}
                  />
                ))} */}
            </ScrollArea>
          </div>
        </CardContent>
      </Card>
    </div>
  )
}
