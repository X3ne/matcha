'use client'
import api, { type ProfileTag } from '@/api'
import { Orientation, Gender, Location } from '@/api/spec'
import { DateTimePicker } from '@/components/datetime-picker'
import TagSelector from '@/components/tag-selector'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
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
import { createLazyFileRoute } from '@tanstack/react-router'
import { useNavigate } from '@tanstack/react-router'
import { subYears, format } from 'date-fns'
import { Trash } from 'lucide-react'
import { useState, useEffect, useRef } from 'react'

export const Route = createLazyFileRoute('/onboarding/')({
  component: OnboardingPage
})

export default function OnboardingPage() {
  const { toast } = useToast()
  const navigate = useNavigate()
  const { user, userProfile } = useUser()

  const fileInputRefs = useRef<(HTMLInputElement | null)[]>([])

  const today = new Date()
  const [birthDate, setBirthDate] = useState<Date | undefined>(
    subYears(today, 18)
  )
  const minDate = subYears(today, 99)
  const maxDate = subYears(today, 18)

  useEffect(() => {
    console.log('userProfile', userProfile)
    if (userProfile) {
      // navigate({ to: '/search' })
      window.location.href = '/search'
    }
  }, [userProfile, navigate])

  const [formData, setFormData] = useState<{
    biography: string
    gender: Gender
    sexualOrientation: Orientation
    min_age: number
    max_age: number
    max_distance_km: number
  }>({
    biography: '',
    gender: Gender.Male,
    sexualOrientation: Orientation.Bisexual,
    min_age: 18,
    max_age: 99,
    max_distance_km: 100
  })

  const [location, setLocation] = useState<Location | null>(null)
  const [selectedTags, setSelectedTags] = useState<ProfileTag[]>([])

  const [pictures, setPictures] = useState<
    { file: File | null; preview: string }[]
  >(Array.from({ length: 5 }, () => ({ file: null, preview: '' })))

  const [isSubmitting, setIsSubmitting] = useState(false)
  const [errorMessage, setErrorMessage] = useState('')

  const { mutateAsync: updateTags } = useMutation({
    mutationFn: async (tags: ProfileTag[]) => {
      if (!tags.length) return
      const tagIds = tags.map((t) => t.id.toString())
      await api.v1.bulkAddTagToMyProfile({ tag_ids: tagIds })
    },
    onSuccess: () => {
      toast({
        title: 'Tags updated!',
        description: 'Your profile tags have been updated successfully.'
      })
    },
    onError: (err: any) => {
      toast({
        title: 'Failed to update tags',
        description: err.message || 'An error occurred while updating tags.',
        variant: 'destructive'
      })
    }
  })

  async function handleSubmit() {
    setIsSubmitting(true)
    setErrorMessage('')

    if (!birthDate) {
      setErrorMessage('Birth date is required.')
      setIsSubmitting(false)
      return
    }
    if (!formData.biography.trim()) {
      setErrorMessage('Biography is required.')
      setIsSubmitting(false)
      return
    }
    const hasAtLeastOnePic = pictures.some((p) => p.file !== null)

    if (!hasAtLeastOnePic) {
      setErrorMessage('You must upload at least one picture.')
      setIsSubmitting(false)
      return
    }

    try {
      const fd = new FormData()

      pictures.forEach(({ file }) => {
        if (file) {
          fd.append('pictures', file)
        }
      })

      const profileObj = {
        bio: formData.biography,
        birth_date: format(birthDate, 'yyyy-MM-dd'),
        name: `${user?.first_name} ${user?.last_name}`,
        avatar_index: 0,
        gender: formData.gender,
        sexual_orientation: formData.sexualOrientation,
        location: location || null,
        min_age: formData.min_age,
        max_age: formData.max_age,
        max_distance_km: formData.max_distance_km
      }

      fd.append(
        'profile',
        new Blob([JSON.stringify(profileObj)], { type: 'application/json' })
      )

      const response = await fetch(
        'https://matcha.abastos.dev/v1/users/@me/onboarding',
        {
          method: 'POST',
          credentials: 'include',
          body: fd
        }
      )

      if (!response.ok) {
        const errorText = await response.text()
        throw new Error(`Error ${response.status}: ${errorText}`)
      }

      if (selectedTags.length > 0) {
        await updateTags(selectedTags)
      }

      toast({
        title: 'Profile Created',
        description:
          'Your profile has been created successfully. You can now start matching with other users.',
        variant: 'default'
      })

      navigate({ to: '/search' })
    } catch (err: any) {
      setErrorMessage(err.message || String(err))
    } finally {
      setIsSubmitting(false)
    }
  }

  function handleInputChange(
    e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>
  ) {
    setFormData({ ...formData, [e.target.name]: e.target.value })
  }

  function handleSelectChange(
    field: 'gender' | 'sexualOrientation',
    value: string
  ) {
    setFormData((prev) => ({
      ...prev,
      [field]: value as Gender & Orientation
    }))
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
      },
      () => {
        alert('Unable to retrieve location.')
      }
    )
  }

  function handleDeleteClick(index: number) {
    return (e: React.MouseEvent<HTMLButtonElement>) => {
      e.stopPropagation()

      setPictures((prev) => {
        const copy = [...prev]
        const pic = copy[index]
        if (pic.preview) {
          URL.revokeObjectURL(pic.preview)
        }
        copy[index] = { file: null, preview: '' }
        return copy
      })

      if (fileInputRefs.current[index]) {
        fileInputRefs.current[index]!.value = ''
      }
    }
  }

  function toggleTag(tag: ProfileTag) {
    setSelectedTags((prev) => {
      const exists = prev.some((t) => t.id === tag.id)
      return exists ? prev.filter((t) => t.id !== tag.id) : [...prev, tag]
    })
  }

  function handlePictureChange(
    e: React.ChangeEvent<HTMLInputElement>,
    index: number
  ) {
    const file = e.target.files?.[0]
    if (file) {
      const preview = URL.createObjectURL(file)
      setPictures((prev) => {
        const copy = [...prev]
        copy[index] = { file, preview }
        return copy
      })
    }
  }

  return (
    <div className="flex items-center justify-center p-4">
      <Card className="w-full max-w-3xl">
        <CardHeader>
          <CardTitle className="text-2xl font-semibold">
            Complete Your Profile
          </CardTitle>
        </CardHeader>
        <CardContent>
          <div className="flex flex-col gap-6 md:flex-row">
            <div className="flex-1 space-y-4">
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
                    <SelectItem value={Orientation.Bisexual}>
                      Bisexual
                    </SelectItem>
                  </SelectContent>
                </Select>
              </div>

              <div>
                <Label htmlFor="birthDate">Birth Date</Label>
                <DateTimePicker
                  value={birthDate}
                  onChange={setBirthDate}
                  min={minDate}
                  max={maxDate}
                  hideTime={true}
                  dateFormat="yyyy-MM-dd"
                />
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

              <div className="flex flex-col gap-1">
                <Label>Your Location</Label>
                <Button
                  size={'sm'}
                  variant="outline"
                  onClick={handleGetLocation}
                >
                  Get My Location
                </Button>
                {location && (
                  <p className="text-xs text-gray-700">
                    Lat: {location.latitude}, Lng: {location.longitude}
                  </p>
                )}
              </div>

              <div>
                <Label>Interests</Label>
                <TagSelector
                  selectedTags={selectedTags}
                  onToggleTag={toggleTag}
                />
                {selectedTags.length > 0 && (
                  <div className="mt-2 flex flex-wrap gap-2">
                    {selectedTags.map((tag) => (
                      <span
                        key={tag.id}
                        className="inline-flex cursor-pointer items-center rounded-full border border-white/20 bg-black/80 px-2 py-1 text-[8px] font-normal text-white"
                      >
                        {tag.name.split('_')[0]}
                      </span>
                    ))}
                  </div>
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
                  onClick={() => fileInputRefs.current[index]?.click()}
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
                    type="file"
                    accept="image/*"
                    className="hidden"
                    onChange={(e) => handlePictureChange(e, index)}
                  />
                </div>
              ))}
            </div>
          </div>

          <Button
            className="mt-6 w-full"
            onClick={handleSubmit}
            disabled={isSubmitting}
          >
            Save Profile
          </Button>

          {errorMessage && (
            <p className="mt-2 text-center text-xs text-red-600">
              {errorMessage}
            </p>
          )}
        </CardContent>
      </Card>
    </div>
  )
}
