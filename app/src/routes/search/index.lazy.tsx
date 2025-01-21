import api from '@/api'
import { UserProfileSortBy, UserProfile, SortOrder } from '@/api/spec'
// Import the TagSelector component
import TagSelector from '@/components/tag-selector'
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
import { Slider } from '@/components/ui/slider'
import { cn } from '@/lib/utils'
import { useQuery } from '@tanstack/react-query'
import { Link } from '@tanstack/react-router'
import { createLazyFileRoute } from '@tanstack/react-router'
import { useState, useEffect } from 'react'
import { FaMapMarkerAlt } from 'react-icons/fa'
import { FaHeart, FaFire, FaFilter } from 'react-icons/fa6'

export const Route = createLazyFileRoute('/search/')({
  component: Search
})

function Search() {
  // Which sorting method is selected?
  const [sortOption, setSortOption] = useState<UserProfileSortBy>(
    UserProfileSortBy.Distance
  )

  // Whether the dialog with filters is open
  const [isDialogOpen, setIsDialogOpen] = useState(false)

  // Whether we're in "advanced search" mode vs "basic search"
  const [isAdvancedSearch, setIsAdvancedSearch] = useState(false)

  /**
   * Filters state. Notice:
   * - `commonTags` and `multipleTags` are now arrays of strings
   *   (instead of comma-separated strings)
   */
  const [filters, setFilters] = useState({
    age: '',
    radius_km: '',
    fameRating: '',
    commonTags: [] as string[], // For basic search
    minAge: '',
    maxAge: '',
    minFame: '',
    maxFame: '',
    multipleTags: [] as string[] // For advanced search
  })

  // The filters we actually apply when "Apply Filters" is clicked
  const [activeFilters, setActiveFilters] = useState(filters)

  // Helper for toggling a tag in the `commonTags` array (basic)
  const handleToggleCommonTag = (tag: string) => {
    setFilters((prev) => {
      const alreadySelected = prev.commonTags.includes(tag)
      const newTags = alreadySelected
        ? prev.commonTags.filter((t) => t !== tag)
        : [...prev.commonTags, tag]
      return { ...prev, commonTags: newTags }
    })
  }

  // Helper for toggling a tag in the `multipleTags` array (advanced)
  const handleToggleMultipleTag = (tag: string) => {
    setFilters((prev) => {
      const alreadySelected = prev.multipleTags.includes(tag)
      const newTags = alreadySelected
        ? prev.multipleTags.filter((t) => t !== tag)
        : [...prev.multipleTags, tag]
      return { ...prev, multipleTags: newTags }
    })
  }

  // Input change handler (for text fields only)
  const handleFilterChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value } = e.target
    setFilters((prevFilters) => ({
      ...prevFilters,
      [name]: value
    }))
  }

  // Toggles between basic and advanced search and resets all fields
  const handleAdvancedSearchToggle = () => {
    setFilters({
      age: '',
      radius_km: '',
      fameRating: '',
      commonTags: [],
      minAge: '',
      maxAge: '',
      minFame: '',
      maxFame: '',
      multipleTags: []
    })
    setIsAdvancedSearch((prev) => !prev)
  }

  // Query: fetch user profiles with current active filters + sort
  const { data: users = [], refetch } = useQuery({
    queryKey: ['users', activeFilters, sortOption],
    retry: false,
    queryFn: async () => {
      const query: Record<string, any> = {
        limit: 25,
        sort_by: sortOption,
        sort_order: SortOrder.Asc,
        // Basic search: single values
        ...(activeFilters.age && {
          min_age: parseInt(activeFilters.age),
          max_age: parseInt(activeFilters.age)
        }),
        ...(activeFilters.fameRating && {
          min_fame_rating: parseInt(activeFilters.fameRating),
          max_fame_rating: parseInt(activeFilters.fameRating)
        }),
        ...(activeFilters.radius_km && {
          radius_km: parseInt(activeFilters.radius_km)
        }),
        // Basic search: tags (commonTags)
        ...(activeFilters.commonTags.length > 0 && {
          common_tags: activeFilters.commonTags
        }),

        // Advanced search: range values
        ...(activeFilters.minAge && {
          min_age: parseInt(activeFilters.minAge)
        }),
        ...(activeFilters.maxAge && {
          max_age: parseInt(activeFilters.maxAge)
        }),
        ...(activeFilters.minFame && {
          min_fame_rating: parseInt(activeFilters.minFame)
        }),
        ...(activeFilters.maxFame && {
          max_fame_rating: parseInt(activeFilters.maxFame)
        }),
        // Advanced search: multipleTags
        ...(activeFilters.multipleTags.length > 0 && {
          tag_ids: activeFilters.multipleTags
        })
      }

      const response = await api.v1.searchProfile(query)
      return response.data
    },
    enabled: true
  })

  // Refetch whenever the user changes the sort dropdown
  useEffect(() => {
    refetch()
  }, [sortOption, refetch])

  // Apply current filters to the "activeFilters" and close the dialog
  const applyFilters = () => {
    setActiveFilters(filters)
    refetch()
    setIsDialogOpen(false)
    console.log('Filters applied:', filters, 'Sorting applied:', sortOption)
  }

  return (
    <div className="container w-full px-0">
      <div className="mb-4 flex justify-between">
        {/* Sorting dropdown */}
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

        {/* Filter Dialog */}
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
                  : 'Basic filters: single age, radius, etc.'}
              </DialogDescription>
            </DialogHeader>

            {/* BASIC SEARCH */}
            {!isAdvancedSearch && (
              <div className="space-y-4">
                {/* Single Age slider */}
                <div>
                  <p className="mb-2 text-sm font-medium">Age</p>
                  <Slider
                    value={[parseInt(filters.age) || 25]}
                    min={18}
                    max={99}
                    step={1}
                    onValueChange={(val) => {
                      setFilters((prev) => ({
                        ...prev,
                        age: val[0].toString()
                      }))
                    }}
                    className={cn('w-full')}
                  />
                  <p className="mt-1 text-xs">
                    Selected Age: {filters.age || '25'}
                  </p>
                </div>

                {/* Single Fame slider */}
                <div>
                  <p className="mb-2 text-sm font-medium">Fame Rating</p>
                  <Slider
                    value={[parseInt(filters.fameRating) || 0]}
                    min={0}
                    max={1000}
                    step={1}
                    onValueChange={(val) => {
                      setFilters((prev) => ({
                        ...prev,
                        fameRating: val[0].toString()
                      }))
                    }}
                    className={cn('w-full')}
                  />
                  <p className="mt-1 text-xs">
                    Selected Fame: {filters.fameRating || '0'}
                  </p>
                </div>

                {/* Single Distance slider */}
                <div>
                  <p className="mb-2 text-sm font-medium">Distance (km)</p>
                  <Slider
                    value={[parseInt(filters.radius_km) || 0]}
                    min={0}
                    max={300}
                    step={5}
                    onValueChange={(val) => {
                      setFilters((prev) => ({
                        ...prev,
                        radius_km: val[0].toString()
                      }))
                    }}
                    className={cn('w-full')}
                  />
                  <p className="mt-1 text-xs">
                    Max Distance: {filters.radius_km || '0'} km
                  </p>
                </div>

                {/* TagSelector for basic tags (commonTags) */}
                <div>
                  <p className="mb-2 text-sm font-medium">Multiple Tags</p>
                  <TagSelector
                    selectedTags={filters.multipleTags}
                    onToggleTag={handleToggleMultipleTag}
                  />
                  <div className="mt-2 flex flex-wrap gap-2">
                    <div className="mt-2 flex flex-wrap gap-2">
                      {filters.multipleTags.map((tag) => (
                        <span
                          key={tag}
                          className="inline-flex cursor-pointer items-center rounded-full border border-white/20 bg-black/80 px-2 py-1 text-[10px] font-normal text-white"
                        >
                          {tag}
                        </span>
                      ))}
                    </div>
                  </div>
                </div>
              </div>
            )}

            {/* ADVANCED SEARCH */}
            {isAdvancedSearch && (
              <div className="space-y-4">
                {/* Double Age slider */}
                <div>
                  <p className="mb-2 text-sm font-medium">Age Range</p>
                  <Slider
                    value={[
                      parseInt(filters.minAge) || 18,
                      parseInt(filters.maxAge) || 30
                    ]}
                    min={18}
                    max={99}
                    step={1}
                    onValueChange={(vals) => {
                      setFilters((prev) => ({
                        ...prev,
                        minAge: vals[0].toString(),
                        maxAge: vals[1].toString()
                      }))
                    }}
                    className={cn('w-full')}
                  />
                  <p className="mt-1 text-xs">
                    {filters.minAge || 18} - {filters.maxAge || 30} yrs
                  </p>
                </div>

                {/* Double Fame slider */}
                <div>
                  <p className="mb-2 text-sm font-medium">Fame Rating Range</p>
                  <Slider
                    value={[
                      parseInt(filters.minFame) || 0,
                      parseInt(filters.maxFame) || 100
                    ]}
                    min={0}
                    max={1000}
                    step={1}
                    onValueChange={(vals) => {
                      setFilters((prev) => ({
                        ...prev,
                        minFame: vals[0].toString(),
                        maxFame: vals[1].toString()
                      }))
                    }}
                    className={cn('w-full')}
                  />
                  <p className="mt-1 text-xs">
                    {filters.minFame || 0} - {filters.maxFame || 100}
                  </p>
                </div>

                {/* Single Distance slider */}
                <div>
                  <p className="mb-2 text-sm font-medium">Distance (km)</p>
                  <Slider
                    value={[parseInt(filters.radius_km) || 0]}
                    min={0}
                    max={300}
                    step={5}
                    onValueChange={(vals) => {
                      setFilters((prev) => ({
                        ...prev,
                        radius_km: vals[0].toString()
                      }))
                    }}
                    className={cn('w-full')}
                  />
                  <p className="mt-1 text-xs">
                    {filters.radius_km || 0} km max
                  </p>
                </div>

                {/* TagSelector for advanced tags (multipleTags) */}
                <div>
                  <p className="mb-2 text-sm font-medium">Multiple Tags</p>
                  <TagSelector
                    selectedTags={filters.multipleTags}
                    onToggleTag={handleToggleMultipleTag}
                  />
                  <div className="mt-2 flex flex-wrap gap-2">
                    <div className="mt-2 flex flex-wrap gap-2">
                      {filters.multipleTags.map((tag) => (
                        <span
                          key={tag}
                          className="inline-flex cursor-pointer items-center rounded-full border border-white/20 bg-black/80 px-2 py-1 text-[10px] font-normal text-white"
                        >
                          {tag}
                        </span>
                      ))}
                    </div>
                  </div>
                </div>
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

      {/* Render the fetched user list */}
      <div className="flex w-full flex-wrap justify-between gap-6">
        {users.map((user) => (
          <UserCard key={user.id} user={user} />
        ))}
      </div>
    </div>
  )
}

/**
 * Card component for displaying each user.
 * This remains unchanged from your original code.
 */
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
        backgroundImage: `url(${
          user.avatar_url
            ? import.meta.env.VITE_API_URL + user.avatar_url
            : 'https://bonnierpublications.com/app/uploads/2022/05/woman-1-480x630.jpg'
        })`,
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
                <p className="text-[8px] font-light">{user.fame_rating}</p>
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

export default Search
