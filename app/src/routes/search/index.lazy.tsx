import api from '@/api'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
  DialogTrigger,
  DialogFooter
} from '@/components/ui/dialog'
import { Input } from '@/components/ui/input'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue
} from '@/components/ui/select'
import { UserContext } from '@/providers/userProvider'
import { useQuery } from '@tanstack/react-query'
import { createLazyFileRoute } from '@tanstack/react-router'
import { Link } from '@tanstack/react-router'
import { useState, useContext } from 'react'
import { FaMapMarkerAlt } from 'react-icons/fa'
import { FaHeart, FaFire, FaFilter } from 'react-icons/fa6'

export const Route = createLazyFileRoute('/search/')({
  component: Search
})

function Search() {
  const { userProfile } = useContext(UserContext)
  const currentUserSexualOrientation =
    userProfile?.sexual_orientation || 'Bisexual'

  const [sortOption, setSortOption] = useState('location')
  const [isDialogOpen, setIsDialogOpen] = useState(false)
  const [isAdvancedSearch, setIsAdvancedSearch] = useState(false)

  const [filters, setFilters] = useState({
    age: '',
    location: '',
    fameRating: '',
    commonTags: '',

    // Advanced fields
    minAge: '',
    maxAge: '',
    minFame: '',
    maxFame: '',
    multipleTags: ''
  })

  const handleFilterChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value } = e.target
    setFilters((prevFilters) => ({
      ...prevFilters,
      [name]: value
    }))
  }

  const handleAdvancedSearchToggle = () => {
    setIsAdvancedSearch((prev) => !prev)
  }

  const applyFilters = () => {
    refetch({ queryKey: ['users', filters] })
    console.log('Filters to apply:', filters)
  }

  // const { data: users = [], refetch } = useQuery(
  //   ['users', filters],
  //   async () => {
  //     const response = await api.v1.getUsers({ params: {
  //       sort: sortOption,
  //       // basic
  //       age: filters.age,
  //       location: filters.location,
  //       fameRating: filters.fameRating,
  //       commonTags: filters.commonTags,
  //       // advanced
  //       minAge: filters.minAge,
  //       maxAge: filters.maxAge,
  //       minFame: filters.minFame,
  //       maxFame: filters.maxFame,
  //       multipleTags: filters.multipleTags,
  //     }})
  //     return response.data
  //   },
  //   {
  //     enabled: false // Disabling auto refetch for demonstration
  //   }
  // )

  const mockUsers = [
    {
      id: '1',
      username: 'Alice',
      age: 25,
      sexual_orientation: 'Bisexual',
      city: 'New York',
      score: 42,
      profilePicture:
        'https://bonnierpublications.com/app/uploads/2022/05/woman-1-480x630.jpg',
      badges: [
        'Yoga Enthusiast',
        'Cat Lover',
        'Tech Geek',
        'Traveler',
        'Foodie'
      ]
    },
    {
      id: '2',
      username: 'Bob',
      age: 30,
      sexual_orientation: 'Heterosexual',
      city: 'Los Angeles',
      score: 30,
      profilePicture:
        'https://bonnierpublications.com/app/uploads/2022/05/woman-1-480x630.jpg',
      badges: ['Dog Lover', 'Gamer', 'Fitness Buff', 'Photographer', 'Surfer']
    },
    {
      id: '3',
      username: 'Charlie',
      age: 18,
      sexual_orientation: 'Homosexual',
      city: 'Chicago',
      score: 55,
      profilePicture:
        'https://bonnierpublications.com/app/uploads/2022/05/woman-1-480x630.jpg',
      badges: ['Student', 'Musician', 'Artist', 'Blogger', 'Runner']
    },
    {
      id: '4',
      username: 'Diana',
      age: 42,
      sexual_orientation: 'Heterosexual',
      city: 'Miami',
      score: 60,
      profilePicture:
        'https://bonnierpublications.com/app/uploads/2022/05/woman-1-480x630.jpg',
      badges: ['Chef', 'Gardener', 'Swimmer', 'Cyclist', 'Bookworm']
    },
    {
      id: '5',
      username: 'Eve',
      age: 56,
      sexual_orientation: 'Bisexual',
      city: 'San Francisco',
      score: 75,
      profilePicture:
        'https://bonnierpublications.com/app/uploads/2022/05/woman-1-480x630.jpg',
      badges: ['Painter', 'Writer', 'Volunteer', 'Photographer', 'Traveler']
    },
    {
      id: '6',
      username: 'Alan',
      age: 52,
      sexual_orientation: 'Bisexual',
      city: 'San Francisco',
      score: 75,
      profilePicture:
        'https://bonnierpublications.com/app/uploads/2022/05/woman-1-480x630.jpg',
      badges: ['Painter', 'Writer', 'Volunteer', 'Photographer', 'Traveler']
    }
  ]

  return (
    <div className="container w-full px-0">
      <div className="mb-4 flex justify-between">
        <Select onValueChange={setSortOption} defaultValue="location">
          <SelectTrigger className="w-fit gap-3">
            <SelectValue placeholder="Sort by" />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="location">Location</SelectItem>
            <SelectItem value="age">Age</SelectItem>
            <SelectItem value="fame_rating">Fame</SelectItem>
            <SelectItem value="common_tags">Tags</SelectItem>
          </SelectContent>
        </Select>

        <Dialog open={isDialogOpen} onOpenChange={setIsDialogOpen}>
          <DialogTrigger asChild>
            <Button variant="secondary" onClick={() => setIsDialogOpen(true)}>
              <FaFilter /> Filter
            </Button>
          </DialogTrigger>

          <DialogContent>
            <DialogHeader>
              <DialogTitle>Filter Users</DialogTitle>
              <DialogDescription>
                {isAdvancedSearch
                  ? 'Advanced criteria: set ranges and multiple tags.'
                  : 'Basic filters: single age, location, etc.'}
              </DialogDescription>
            </DialogHeader>

            {!isAdvancedSearch && (
              <div className="space-y-4">
                <Input
                  name="age"
                  value={filters.age}
                  onChange={handleFilterChange}
                  placeholder="Enter age"
                />
                <Input
                  name="fameRating"
                  value={filters.fameRating}
                  onChange={handleFilterChange}
                  placeholder="Enter fame rating"
                />
                <Input
                  name="location"
                  value={filters.location}
                  onChange={handleFilterChange}
                  placeholder="Enter location"
                />
                <Input
                  name="commonTags"
                  value={filters.commonTags}
                  onChange={handleFilterChange}
                  placeholder="Interest tags (comma-separated)"
                />
              </div>
            )}

            {isAdvancedSearch && (
              <div className="space-y-4">
                <div className="flex gap-2">
                  <Input
                    name="minAge"
                    value={filters.minAge}
                    onChange={handleFilterChange}
                    placeholder="Min Age"
                  />
                  <Input
                    name="maxAge"
                    value={filters.maxAge}
                    onChange={handleFilterChange}
                    placeholder="Max Age"
                  />
                </div>
                <div className="flex gap-2">
                  <Input
                    name="minFame"
                    value={filters.minFame}
                    onChange={handleFilterChange}
                    placeholder="Min Fame"
                  />
                  <Input
                    name="maxFame"
                    value={filters.maxFame}
                    onChange={handleFilterChange}
                    placeholder="Max Fame"
                  />
                </div>
                <Input
                  name="location"
                  value={filters.location}
                  onChange={handleFilterChange}
                  placeholder="Location"
                />
                <Input
                  name="multipleTags"
                  value={filters.multipleTags}
                  onChange={handleFilterChange}
                  placeholder="Interest tags (comma-separated)"
                />
              </div>
            )}

            <DialogFooter className="flex justify-between">
              <Button variant="outline" onClick={handleAdvancedSearchToggle}>
                {isAdvancedSearch ? 'Basic Search' : 'Advanced Search'}
              </Button>
              <Button onClick={applyFilters}>Apply Filters</Button>
            </DialogFooter>
          </DialogContent>
        </Dialog>
      </div>

      <div className="flex w-full flex-wrap justify-between gap-6">
        {mockUsers.map((user) => (
          <UserCard key={user.id} user={user} />
        ))}
      </div>
    </div>
  )
}

function UserCard({
  user
}: {
  user: {
    id: string
    username: string
    age: number
    sexual_orientation: string
    city: string
    score: number
    profilePicture: string
    badges: string[]
  }
}) {
  const [isConfettiActive, setIsConfettiActive] = useState(false)

  const handleLikeClick = (e: React.MouseEvent<HTMLButtonElement>) => {
    e.preventDefault()
    e.stopPropagation()

    setIsConfettiActive(true)
    setTimeout(() => {
      setIsConfettiActive(false)
    }, 1000)
  }

  return (
    <Link
      to="/profile/$username"
      params={{ username: user.username }}
      className="motion-preset-slide-down mx-auto flex h-72 w-56 flex-col justify-end overflow-hidden rounded-xl shadow sm:mx-0"
      style={{
        backgroundImage: `url(${user.profilePicture})`,
        backgroundSize: 'cover',
        backgroundPosition: 'center'
      }}
    >
      <div className="bg-gradient-to-t from-black/80 to-transparent pt-4">
        <div className="flex items-center justify-between px-4">
          <div className="flex flex-col text-white">
            <h2 className="text-lg font-semibold uppercase">
              {user.username}, {user.age}
            </h2>
            <ul className="gap flex flex-col gap-1">
              <li className="flex items-center gap-1">
                <FaMapMarkerAlt size={12} />
                <p className="text-xs font-light">{user.city}</p>
              </li>
              <li className="flex items-center gap-1">
                <FaFire size={12} />
                <p className="text-xs font-light">{user.score}</p>
              </li>
            </ul>
          </div>

          <Button
            size="icon"
            className={`rounded-full border-rose-600 bg-rose-600 p-5 shadow hover:border-rose-700 hover:bg-rose-700 ${
              isConfettiActive
                ? 'motion-preset-confetti motion-duration-700'
                : ''
            }`}
            onClick={handleLikeClick}
          >
            <FaHeart color="white" />
          </Button>
        </div>
        <div className="mt-2 flex flex-wrap gap-1 px-4 pb-4">
          {user.badges.slice(0, 2).map((badge, index) => (
            <Badge
              key={index}
              variant="secondary"
              className="rounded-full border border-white/20 bg-black/80 py-1 text-[8px] font-normal text-white hover:bg-black/80"
            >
              {badge}
            </Badge>
          ))}
        </div>
      </div>
    </Link>
  )
}

export default UserCard
