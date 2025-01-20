'use client'
import { Orientation, Gender, Location } from '@/api/spec'
import TagSelector from '@/components/tag-selector'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue
} from '@/components/ui/select'
import { Textarea } from '@/components/ui/textarea'
import { useToast } from '@/components/ui/use-toast'
import { useUser } from '@/hooks/useUser'
import { createLazyFileRoute } from '@tanstack/react-router'
import { useNavigate } from '@tanstack/react-router'
import { useState, useEffect } from 'react'

export const Route = createLazyFileRoute('/onboarding/')({
  component: OnboardingPage
})

export default function OnboardingPage() {
  const { toast } = useToast()

  const navigate = useNavigate()
  const { user, userProfile } = useUser()

  useEffect(() => {
    console.log('userProfile', userProfile)
    if (userProfile) {
      navigate({ to: '/search' })
    }
  }, [userProfile, navigate])

  const [formData, setFormData] = useState<{
    age: string
    biography: string
    gender: Gender
    sexualOrientation: Orientation
  }>({
    age: '',
    biography: '',
    gender: Gender.Male,
    sexualOrientation: Orientation.Bisexual
  })

  const [location, setLocation] = useState<Location | null>(null)
  const [selectedTags, setSelectedTags] = useState<string[]>([])

  const [pictures, setPictures] = useState<
    { file: File | null; preview: string }[]
  >(Array(5).fill({ file: null, preview: '' }))

  const [isSubmitting, setIsSubmitting] = useState(false)
  const [errorMessage, setErrorMessage] = useState('')

  async function handleSubmit() {
    setIsSubmitting(true)
    setErrorMessage('')

    if (!formData.age.trim()) {
      setErrorMessage('Age is required.')
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
        age: Number(formData.age),
        name: `${user?.first_name} ${user?.last_name}`,
        avatar_index: 0,
        gender: formData.gender,
        sexual_orientation: formData.sexualOrientation,
        location: location || null,
        tag_ids: []
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
      toast({
        title: 'Profile Created',
        description:
          'Your profile has been created successfully. You can now start matching with other users.',
        variant: 'default'
      })
      window.location.href = '/search'
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

  function toggleTag(tag: string) {
    setSelectedTags((prev) =>
      prev.includes(tag) ? prev.filter((t) => t !== tag) : [...prev, tag]
    )
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
                <Label htmlFor="age">Age</Label>
                <Input
                  id="age"
                  name="age"
                  type="number"
                  value={formData.age}
                  onChange={handleInputChange}
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
                <Button variant="outline" onClick={handleGetLocation}>
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
                <div className="mt-2 flex flex-wrap gap-2">
                  {selectedTags.map((tag) => (
                    <span
                      key={tag}
                      className="inline-flex cursor-pointer items-center rounded-full border border-white/20 bg-black/80 px-2 py-1 text-[10px] font-normal text-white"
                    >
                      #{tag}
                    </span>
                  ))}
                </div>
              </div>
            </div>

            <div className="grid h-fit flex-1 grid-cols-1 gap-4 sm:grid-cols-2">
              {pictures.map((pic, index) => (
                <div
                  key={index}
                  className="flex h-36 w-full cursor-pointer flex-col items-center justify-center rounded-md border-2 border-dashed p-2 text-center"
                  onClick={() =>
                    document.getElementById(`file-input-${index}`)?.click()
                  }
                >
                  {pic.preview ? (
                    <img
                      src={pic.preview}
                      alt={`Picture ${index + 1}`}
                      className="h-full w-full rounded object-cover"
                    />
                  ) : (
                    <span className="text-sm text-gray-500">
                      Picture {index + 1}
                    </span>
                  )}
                  <input
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
