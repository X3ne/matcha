import api from '@/api'
import { UserProfileSortBy, UserProfile, SortOrder } from '@/api/spec'
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
import { useQuery } from '@tanstack/react-query'
import { createLazyFileRoute } from '@tanstack/react-router'
import { Link } from '@tanstack/react-router'
import { useState, useEffect } from 'react'
import { FaMapMarkerAlt } from 'react-icons/fa'
import { FaHeart, FaFire, FaFilter } from 'react-icons/fa6'

export const Route = createLazyFileRoute('/search/')({
  component: Search
})

function Search() {
  const [sortOption, setSortOption] = useState<UserProfileSortBy>(
    UserProfileSortBy.Distance
  )
  const [isDialogOpen, setIsDialogOpen] = useState(false)
  const [isAdvancedSearch, setIsAdvancedSearch] = useState(false)
  const [filters, setFilters] = useState({
    age: '',
    location: '',
    fameRating: '',
    commonTags: '',
    minAge: '',
    maxAge: '',
    minFame: '',
    maxFame: '',
    multipleTags: ''
  })

  const [activeFilters, setActiveFilters] = useState(filters)

  const handleFilterChange = (e) => {
    const { name, value } = e.target
    setFilters((prevFilters) => ({
      ...prevFilters,
      [name]: value
    }))
  }

  const handleAdvancedSearchToggle = () => {
    setFilters({
      age: '',
      location: '',
      fameRating: '',
      commonTags: '',
      minAge: '',
      maxAge: '',
      minFame: '',
      maxFame: '',
      multipleTags: ''
    })
    setIsAdvancedSearch((prev) => !prev)
  }

  const { data: users = [], refetch } = useQuery({
    queryKey: ['users', activeFilters, sortOption],
    retry: false,
    queryFn: async () => {
      const query: Record<string, any> = {
        limit: 25,
        sort_by: sortOption,
        ...(activeFilters.age && {
          min_age: parseInt(activeFilters.age),
          max_age: parseInt(activeFilters.age)
        }),
        sort_order: SortOrder.Desc,
        ...(activeFilters.fameRating && {
          min_fame_rating: parseInt(activeFilters.fameRating),
          max_fame_rating: parseInt(activeFilters.fameRating)
        }),
        ...(activeFilters.location && { location: activeFilters.location }),
        ...(activeFilters.commonTags && {
          common_tags: activeFilters.commonTags
            .split(',')
            .map((tag) => tag.trim())
        }),
        ...(activeFilters.maxAge && {
          max_age: parseInt(activeFilters.maxAge)
        }),
        ...(activeFilters.minAge && {
          min_age: parseInt(activeFilters.minAge)
        }),
        ...(activeFilters.maxFame && {
          max_fame_rating: parseInt(activeFilters.maxFame)
        }),
        ...(activeFilters.minFame && {
          min_fame_rating: parseInt(activeFilters.minFame)
        }),
        ...(activeFilters.multipleTags && {
          tag_ids: activeFilters.multipleTags
            .split(',')
            .map((tag) => tag.trim())
        })
      }

      const response = await api.v1.searchProfile(query)
      return response.data
    },
    enabled: true
  })

  useEffect(() => {
    refetch()
  }, [sortOption, refetch])

  const applyFilters = () => {
    setActiveFilters(filters)
    refetch()
    setIsDialogOpen(false)
    console.log('Filters applied:', filters, 'Sorting applied:', sortOption)
  }

  return (
    <div className="container w-full px-0">
      <div className="mb-4 flex justify-between">
        <Select
          onValueChange={(value) => {
            setSortOption(value as UserProfileSortBy)
          }}
          defaultValue={UserProfileSortBy.Distance}
        >
          <SelectTrigger className="w-fit gap-3">
            <SelectValue placeholder="Sort by" />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value={UserProfileSortBy.Distance}>Distance</SelectItem>
            <SelectItem value={UserProfileSortBy.Age}>Age</SelectItem>
            <SelectItem value={UserProfileSortBy.FameRating}>Fame</SelectItem>
            <SelectItem value={UserProfileSortBy.Tags}>Tags</SelectItem>
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
              <DialogDescription className="text-xs">
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
        {users.map((user) => (
          <UserCard key={user.id} user={user} />
        ))}
      </div>
    </div>
  )
}

function UserCard({ user }: { user: UserProfile }) {
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
      to="/profile/$id"
      params={{ id: user.id }}
      className="motion-preset-slide-down mx-auto flex h-72 w-56 flex-col justify-end overflow-hidden rounded-xl shadow sm:mx-0"
      style={{
        backgroundImage: `url(${user.avatar_url ? import.meta.env.VITE_API_URL + user.avatar_url : 'https://bonnierpublications.com/app/uploads/2022/05/woman-1-480x630.jpg'})`,
        backgroundSize: 'cover',
        backgroundPosition: 'center'
      }}
    >
      <div className="bg-gradient-to-t from-black/80 to-transparent pt-4">
        <div className="flex items-center justify-between px-4">
          <div className="flex flex-col text-white">
            <h2 className="text-lg font-semibold uppercase">
              {user.name.split(' ')[0]}, {user.age}
            </h2>
            <ul className="gap flex flex-col gap-1">
              <li className="flex items-center gap-1">
                <FaMapMarkerAlt size={10} />
                <p className="text-[8px] font-light">
                  {user.approx_distance_km} kilometers away
                </p>
              </li>
              <li className="flex items-center gap-1">
                <FaFire size={10} />
                <p className="text-[8px] font-light">
                  10 {/* {user.fame_rating} */}
                </p>
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
          {user.tags.slice(0, 2).map((tag, index) => (
            <Badge
              key={index}
              variant="secondary"
              className="rounded-full border border-white/20 bg-black/80 py-1 text-[6px] font-normal text-white hover:bg-black/80"
            >
              {tag.name}
            </Badge>
          ))}
        </div>
      </div>
    </Link>
  )
}

export default UserCard
