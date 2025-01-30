import api, { UploadProfilePictureForm, type ProfileTag } from '@/api'
import { Orientation, Gender, Location } from '@/api/spec'
import TagSelector from '@/components/tag-selector'
import { Button } from '@/components/ui/button'
import { DialogClose } from '@/components/ui/dialog'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue
} from '@/components/ui/select'
import { Slider } from '@/components/ui/slider'
import { Textarea } from '@/components/ui/textarea'
import { useToast } from '@/components/ui/use-toast'
import { useUser } from '@/hooks/useUser'
import { cn } from '@/lib/utils'
import { useMutation } from '@tanstack/react-query'
import { Trash } from 'lucide-react'
import { useState, useEffect, useRef } from 'react'

interface PictureInfo {
  file: File | null
  preview: string
  offset: number | null
}

export function EditProfileForm() {
  const { toast } = useToast()
  const { user, userProfile, refreshUser, refreshUserProfile } = useUser()

  const [initialSnapshot, setInitialSnapshot] = useState<{
    formData: any
    location: Location | null
    tags: ProfileTag[]
    pictures: PictureInfo[]
  }>()

  const [formData, setFormData] = useState({
    firstName: '',
    lastName: '',
    email: '',
    biography: '',
    gender: Gender.Male,
    sexualOrientation: Orientation.Bisexual,
    min_age: 18,
    max_age: 100,
    max_distance_km: 150
  })

  const [location, setLocation] = useState<Location | null>(null)
  const [selectedTags, setSelectedTags] = useState<ProfileTag[]>([])
  const [errorMessage, setErrorMessage] = useState('')
  const [isModified, setIsModified] = useState(false)

  const [pictures, setPictures] = useState<PictureInfo[]>(
    Array(5).fill({ file: null, preview: '', offset: null })
  )
  const fileInputRefs = useRef<(HTMLInputElement | null)[]>([])

  const [previousPictureUrls, setPreviousPictureUrls] = useState<string[]>([])

  useEffect(() => {
    if (!userProfile) return

    setPreviousPictureUrls(userProfile.picture_urls)
    setSelectedTags(userProfile.tags)

    const urls = [...userProfile.picture_urls]
    const avatarIndex = urls.indexOf(
      userProfile.avatar_url
        ? userProfile.avatar_url
        : userProfile.picture_urls[0]
    )
    if (avatarIndex > -1) {
      const [avatar] = urls.splice(avatarIndex, 1)
      urls.unshift(avatar)
    }

    const maxPictures = 5
    const newPictures = urls.slice(0, maxPictures).map((url, i) => ({
      file: null,
      preview: import.meta.env.VITE_API_URL + url,
      offset: i
    }))
    while (newPictures.length < maxPictures) {
      newPictures.push({ file: null, preview: '', offset: null })
    }

    const initialFormData = {
      firstName: user?.first_name || '',
      lastName: user?.last_name || '',
      email: user?.email || '',
      biography: userProfile.bio || '',
      gender: userProfile.gender,
      sexualOrientation: userProfile.sexual_orientation,
      min_age: userProfile.min_age,
      max_age: userProfile.max_age,
      max_distance_km: userProfile.max_distance_km
    }

    const initialLocation = null
    const initialTags = [...userProfile.tags]
    const initialPictures = newPictures.map((pic) => ({ ...pic }))

    setInitialSnapshot({
      formData: initialFormData,
      location: initialLocation,
      tags: initialTags,
      pictures: initialPictures
    })

    setPictures(newPictures)

    setFormData({
      firstName: user?.first_name || '',
      lastName: user?.last_name || '',
      email: user?.email || '',
      biography: userProfile.bio || '',
      gender: userProfile.gender,
      sexualOrientation: userProfile.sexual_orientation,
      min_age: userProfile.min_age,
      max_age: userProfile.max_age,
      max_distance_km: userProfile.max_distance_km
    })

    setSelectedTags(userProfile.tags)
  }, [user, userProfile])

  useEffect(() => {
    if (!initialSnapshot) return

    const isFormDataModified =
      JSON.stringify(formData) !== JSON.stringify(initialSnapshot.formData)
    const isLocationModified =
      JSON.stringify(location) !== JSON.stringify(initialSnapshot.location)
    const isTagsModified =
      JSON.stringify(selectedTags) !== JSON.stringify(initialSnapshot.tags)
    const isPicturesModified =
      JSON.stringify(pictures) !== JSON.stringify(initialSnapshot.pictures)

    setIsModified(
      isFormDataModified ||
        isLocationModified ||
        isTagsModified ||
        isPicturesModified
    )
  }, [formData, location, selectedTags, pictures, initialSnapshot])

  useEffect(() => {
    return () => {
      pictures.forEach((pic) => {
        if (pic.file && pic.preview) {
          URL.revokeObjectURL(pic.preview)
        }
      })
    }
  }, [pictures])

  const { mutate: updateEverything, isPending: isSubmitting } = useMutation({
    mutationFn: async () => {
      const userData = {
        email: formData.email,
        first_name: formData.firstName,
        last_name: formData.lastName
      }

      const profileData = {
        name: `${formData.firstName} ${formData.lastName}`,
        bio: formData.biography || null,
        gender: formData.gender,
        sexual_orientation: formData.sexualOrientation,
        location: location || null,
        min_age: formData.min_age,
        max_age: formData.max_age,
        max_distance_km: formData.max_distance_km
      }

      await api.v1.updateMe(userData)
      await api.v1.updateMyProfile(profileData)
    },
    onSuccess: () => {
      toast({
        title: 'Profile updated!',
        description: 'Your profile has been successfully updated.'
      })
      setIsModified(false)
      refreshUser()
      refreshUserProfile()
    },
    onError: (err: any) => {
      setErrorMessage(
        err.message || 'An error occurred while updating the profile.'
      )
    }
  })

  const uploadMutation = useMutation<
    void,
    unknown,
    { file: File; index: number }
  >({
    mutationFn: async ({ file }) => {
      const data: UploadProfilePictureForm = { picture: file }
      return await api.v1.uploadProfilePicture(data)
    },

    onSuccess: async (_, { index }) => {
      await refreshUserProfile()

      if (!userProfile) return
      const newUrls = userProfile.picture_urls
      const oldSet = new Set(previousPictureUrls)
      let newlyAddedUrl: string | null = null

      for (const url of newUrls) {
        if (!oldSet.has(url)) {
          newlyAddedUrl = url
          break
        }
      }
      if (!newlyAddedUrl) return

      const newOffset = userProfile.picture_urls.indexOf(newlyAddedUrl)

      if (index === 0 && newOffset !== -1) {
        await api.v1.setDefaultProfilePicture(newOffset)
        await refreshUserProfile()
      }
    },

    onError: (err: any) => {
      console.error(err)
      toast({
        title: 'Upload failed',
        description:
          err.message || 'Something went wrong uploading the picture.',
        variant: 'destructive'
      })
    }
  })

  const deleteMutation = useMutation({
    mutationFn: async (offset: number) => {
      return api.v1.deleteProfilePicture(offset)
    },
    onSuccess: async () => {
      await refreshUserProfile()
    },
    onError: (err: any) => {
      console.error(err)
      toast({
        title: 'Delete failed',
        description:
          err.message || 'Something went wrong deleting the picture.',
        variant: 'destructive'
      })
    }
  })

  function handleSubmit() {
    setErrorMessage('')

    updateEverything()
    updateTags(selectedTags)
  }

  function handleGetLocation() {
    if (!navigator.geolocation) {
      alert('Geolocation is not supported!')
      return
    }
    navigator.geolocation.getCurrentPosition(
      (pos) => {
        setLocation({
          latitude: pos.coords.latitude,
          longitude: pos.coords.longitude
        })
        setIsModified(true)
      },
      () => {
        alert('Unable to retrieve location.')
      }
    )
  }

  function handleInputChange(
    e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>
  ) {
    setFormData((prev) => {
      const updatedData = { ...prev, [e.target.name]: e.target.value }
      setIsModified(
        JSON.stringify(updatedData) !==
          JSON.stringify({
            firstName: user?.first_name || '',
            lastName: user?.last_name || '',
            email: user?.email || '',
            biography: userProfile?.bio || '',
            gender: userProfile?.gender || Gender.Male,
            sexualOrientation:
              userProfile?.sexual_orientation || Orientation.Bisexual
          })
      )
      return updatedData
    })
  }

  function handleSelectChange(
    field: 'gender' | 'sexualOrientation',
    value: string
  ) {
    setFormData((prev) => {
      const updatedData = { ...prev, [field]: value as Gender & Orientation }
      setIsModified(
        JSON.stringify(updatedData) !==
          JSON.stringify({
            firstName: user?.first_name || '',
            lastName: user?.last_name || '',
            email: user?.email || '',
            biography: userProfile?.bio || '',
            gender: userProfile?.gender || Gender.Male,
            sexualOrientation:
              userProfile?.sexual_orientation || Orientation.Bisexual
          })
      )
      return updatedData
    })
  }

  function handlePictureChange(
    e: React.ChangeEvent<HTMLInputElement>,
    index: number
  ) {
    const file = e.target.files?.[0]
    if (!file) return

    setPictures((prev) => {
      const copy = [...prev]
      if (copy[index].file && copy[index].preview) {
        URL.revokeObjectURL(copy[index].preview)
      }
      copy[index] = {
        file,
        preview: URL.createObjectURL(file),
        offset: copy[index].offset
      }
      return copy
    })
    setIsModified(true)

    uploadMutation.mutate({ file, index })
  }

  function handleDeleteClick(index: number) {
    return (e: React.MouseEvent<HTMLButtonElement>) => {
      e.stopPropagation()

      setPictures((prev) => {
        const copy = [...prev]
        const pic = copy[index]

        if (pic.file && pic.preview) {
          URL.revokeObjectURL(pic.preview)
        }
        copy[index] = { file: null, preview: '', offset: null }
        return copy
      })

      if (fileInputRefs.current[index]) {
        fileInputRefs.current[index]!.value = ''
      }

      const offset = pictures[index].offset
      if (offset !== null && offset !== undefined) {
        deleteMutation.mutate(offset)
      }

      setIsModified(true)
    }
  }

  const { mutate: updateTags } = useMutation({
    mutationFn: async (selectedTags: ProfileTag[]) => {
      if (!userProfile) return

      const currentTagIds = userProfile.tags.map((t) => t.id)
      const selectedTagIds = selectedTags.map((t) => t.id)

      const tagsToAdd = selectedTagIds.filter(
        (id) => !currentTagIds.includes(id)
      )
      const tagsToRemove = currentTagIds.filter(
        (id) => !selectedTagIds.includes(id)
      )

      try {
        if (tagsToAdd.length > 0) {
          await api.v1.bulkAddTagToMyProfile({ tag_ids: tagsToAdd })
        }
        if (tagsToRemove.length > 0) {
          await api.v1.bulkRemoveTagFromMyProfile({ tag_ids: tagsToRemove })
        }

        await refreshUserProfile()
        toast({
          title: 'Tags updated!',
          description: 'Your profile tags have been successfully updated.'
        })
      } catch (err: any) {
        toast({
          title: 'Failed to update tags',
          description: err.message || 'An error occurred while updating tags.',
          variant: 'destructive'
        })
      }
    }
  })

  function toggleTag(tag: ProfileTag) {
    setSelectedTags((prev) => {
      const exists = prev.some((t) => t.id === tag.id)
      return exists ? prev.filter((t) => t.id !== tag.id) : [...prev, tag]
    })
    setIsModified(true)
  }

  return (
    <>
      <div className="mx-px flex flex-col gap-6 md:flex-row">
        <div className="flex-1 space-y-4">
          <div>
            <Label htmlFor="firstName">First Name</Label>
            <Input
              id="firstName"
              name="firstName"
              value={formData.firstName}
              onChange={handleInputChange}
            />
          </div>

          <div>
            <Label htmlFor="lastName">Last Name</Label>
            <Input
              id="lastName"
              name="lastName"
              value={formData.lastName}
              onChange={handleInputChange}
            />
          </div>

          <div>
            <Label htmlFor="email">Email</Label>
            <Input
              id="email"
              name="email"
              type="email"
              value={formData.email}
              onChange={handleInputChange}
            />
          </div>

          <div>
            <Label htmlFor="gender">Gender</Label>
            <Select
              value={formData.gender}
              onValueChange={(val) => handleSelectChange('gender', val)}
            >
              <SelectTrigger>
                <SelectValue placeholder="Select your gender" />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value={Gender.Male}>Male</SelectItem>
                <SelectItem value={Gender.Female}>Female</SelectItem>
              </SelectContent>
            </Select>
          </div>

          <div>
            <Label htmlFor="sexualOrientation">Preferred Partners</Label>
            <Select
              value={formData.sexualOrientation}
              onValueChange={(val) =>
                handleSelectChange('sexualOrientation', val)
              }
            >
              <SelectTrigger>
                <SelectValue placeholder="Select orientation" />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value={Orientation.Male}>Male</SelectItem>
                <SelectItem value={Orientation.Female}>Female</SelectItem>
                <SelectItem value={Orientation.Bisexual}>Bisexual</SelectItem>
              </SelectContent>
            </Select>
          </div>

          <div>
            <Label htmlFor="biography">Biography</Label>
            <Textarea
              id="biography"
              name="biography"
              value={formData.biography}
              onChange={handleInputChange}
            />
          </div>

          <div>
            <Label>Interests</Label>
            <TagSelector selectedTags={selectedTags} onToggleTag={toggleTag} />
            <div className="mt-2 flex flex-wrap gap-2">
              {selectedTags.map((tag) => (
                <span
                  key={tag.id}
                  className="inline-flex cursor-pointer items-center rounded-full border border-white/20 bg-black/80 px-2 py-1 text-[10px] font-normal text-white"
                >
                  {tag.name} {/* Change from tag to tag.name */}
                </span>
              ))}
            </div>
          </div>

          <div className="flex flex-col gap-1">
            <Label>Your Location</Label>
            <Button size="sm" variant="outline" onClick={handleGetLocation}>
              Update My Location
            </Button>
            {location && (
              <p className="text-xs text-gray-700">
                Lat: {location.latitude}, Lng: {location.longitude}
              </p>
            )}
          </div>

          <div>
            <Label>Preferred Age Range</Label>
            <Slider
              value={[formData.min_age, formData.max_age]}
              min={18}
              max={99}
              step={1}
              onValueChange={(vals) => {
                setFormData((prev) => ({
                  ...prev,
                  min_age: vals[0],
                  max_age: vals[1]
                }))
              }}
              className="!mt-1 w-full"
            />
            <p className="mt-1 text-xs">
              {formData.min_age} - {formData.max_age} yrs
            </p>
          </div>

          <div>
            <Label className="mb-1">Match Radius (km)</Label>
            <Slider
              value={[formData.max_distance_km || 150]}
              min={0}
              max={150}
              step={5}
              onValueChange={(val) => {
                setFormData((prev) => ({
                  ...prev,
                  max_distance_km: val[0]
                }))
              }}
              className={cn('!mt-1 w-full')}
            />
            <p className="mt-1 text-xs">
              Max Distance: {formData.max_distance_km || '0'} km
            </p>
          </div>
        </div>

        <div className="grid h-fit w-fit grid-cols-1 gap-4 sm:grid-cols-2">
          {pictures.map((pic, index) => (
            <div
              key={index}
              className={`flex h-36 w-36 cursor-pointer flex-col items-center justify-center rounded-md ${
                pic.preview
                  ? ''
                  : index === 0
                    ? 'border-2 border-dashed border-primary'
                    : 'border-2 border-dashed'
              } text-center`}
              onClick={() =>
                document.getElementById(`file-input-${index}`)?.click()
              }
            >
              {pic.preview ? (
                <div className="relative">
                  <img
                    src={pic.preview}
                    alt={`Picture ${index + 1}`}
                    className="h-36 w-full rounded-md object-cover"
                  />
                  <Button
                    size="icon"
                    className="absolute right-1 top-1 h-6 w-6 rounded-full"
                    aria-label="Delete Picture"
                    onClick={handleDeleteClick(index)}
                  >
                    <Trash className="!size-3" />
                  </Button>
                </div>
              ) : (
                <span className="text-sm text-gray-500">
                  Picture {index + 1}
                </span>
              )}
              <input
                ref={(el) => (fileInputRefs.current[index] = el)}
                id={`file-input-${index}`}
                type="file"
                accept="image/*"
                className="hidden"
                onChange={(e) => handlePictureChange(e, index)}
              />
            </div>
          ))}
        </div>
      </div>

      <DialogClose asChild>
        <Button
          className="mt-6 w-full"
          onClick={handleSubmit}
          disabled={isSubmitting || !isModified}
        >
          Save Changes
        </Button>
      </DialogClose>

      {errorMessage && (
        <p className="mt-2 text-center text-xs text-red-600">{errorMessage}</p>
      )}
    </>
  )
}
