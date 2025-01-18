import { Orientation, Gender, Location } from '@/api/spec'
import TagSelector from '@/components/tag-selector'
import { Button } from '@/components/ui/button'
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
import { UserContext } from '@/providers/userProvider'
import { useContext, useState, useEffect } from 'react'

export function EditProfileForm() {
  const { user, userProfile } = useContext(UserContext)

  const [formData, setFormData] = useState({
    firstName: '',
    lastName: '',
    email: '',
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
  const [isModified, setIsModified] = useState(false)

  useEffect(() => {
    if (user && userProfile) {
      setFormData({
        firstName: user.first_name,
        lastName: user.last_name,
        email: user.email,
        age: userProfile.age.toString(),
        biography: userProfile.bio || '',
        gender: userProfile.gender,
        sexualOrientation: userProfile.sexual_orientation
      })
      setSelectedTags(userProfile.tags.map((tag) => tag.name))
    }
  }, [user, userProfile])

  async function handleSubmit() {
    setIsSubmitting(true)
    setErrorMessage('')

    try {
      const fd = new FormData()

      pictures.forEach(({ file }) => {
        if (file) {
          fd.append('pictures', file)
        }
      })

      const profileObj = {
        firstName: formData.firstName,
        lastName: formData.lastName,
        email: formData.email,
        bio: formData.biography,
        age: Number(formData.age),
        gender: formData.gender,
        sexual_orientation: formData.sexualOrientation,
        location: location || null,
        tag_ids: selectedTags
      }

      fd.append(
        'profile',
        new Blob([JSON.stringify(profileObj)], { type: 'application/json' })
      )

      const response = await fetch(
        'https://api.example.com/v1/users/@me/profile', // Guessed endpoint
        {
          method: 'PUT',
          credentials: 'include',
          body: fd
        }
      )

      if (!response.ok) {
        const errorText = await response.text()
        throw new Error(`Error ${response.status}: ${errorText}`)
      }

      alert('Profile updated successfully!')
      setIsModified(false) // Reset modification flag after successful update
    } catch (err: any) {
      setErrorMessage(
        err.message || 'An error occurred while updating the profile.'
      )
    } finally {
      setIsSubmitting(false)
    }
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
            age: userProfile?.age.toString() || '',
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
            age: userProfile?.age.toString() || '',
            biography: userProfile?.bio || '',
            gender: userProfile?.gender || Gender.Male,
            sexualOrientation:
              userProfile?.sexual_orientation || Orientation.Bisexual
          })
      )
      return updatedData
    })
  }

  function toggleTag(tag: string) {
    setSelectedTags((prev) => {
      const updatedTags = prev.includes(tag)
        ? prev.filter((t) => t !== tag)
        : [...prev, tag]
      setIsModified(
        JSON.stringify(updatedTags.sort()) !==
          JSON.stringify(userProfile?.tags.map((tag) => tag.name).sort() || [])
      )
      return updatedTags
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
        setIsModified(true)
        return copy
      })
    }
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

          <div>
            <Label>Interests</Label>
            <TagSelector selectedTags={selectedTags} onToggleTag={toggleTag} />
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

          <div className="flex flex-col gap-1">
            <Label>Your Location</Label>
            <Button variant="outline" onClick={handleGetLocation}>
              Update My Location
            </Button>
            {location && (
              <p className="text-xs text-gray-700">
                Lat: {location.latitude}, Lng: {location.longitude}
              </p>
            )}
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
        disabled={isSubmitting || !isModified}
      >
        Save Changes
      </Button>

      {errorMessage && (
        <p className="mt-2 text-center text-xs text-red-600">{errorMessage}</p>
      )}
    </>
  )
}
