import api, { type ProfileTag } from '@/api'
import { UserProfileSortBy, UserProfile, SortOrder } from '@/api/spec'
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
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue
} from '@/components/ui/select'
import { Slider } from '@/components/ui/slider'
import { useToast } from '@/components/ui/use-toast'
import { cn } from '@/lib/utils'
import { UserContext } from '@/providers/userProvider'
import { useInfiniteQuery, useMutation } from '@tanstack/react-query'
import { Link } from '@tanstack/react-router'
import { createLazyFileRoute } from '@tanstack/react-router'
import { ListFilter } from 'lucide-react'
import { useContext, useState, useEffect } from 'react'
import { FaMapMarkerAlt } from 'react-icons/fa'
import { FaHeart, FaHeartCrack, FaFire } from 'react-icons/fa6'

export const Route = createLazyFileRoute('/search/')({
  component: Search
})

function Search() {
  const { userProfile: currentUserProfile } = useContext(UserContext)

  const [sortOption, setSortOption] = useState<UserProfileSortBy>(
    UserProfileSortBy.Distance
  )
  const [isDialogOpen, setIsDialogOpen] = useState(false)
  const [isAdvancedSearch, setIsAdvancedSearch] = useState(false)

  const [filters, setFilters] = useState({
    age: currentUserProfile?.age.toString(),
    radius_km: '',
    rating: '',
    minAge: currentUserProfile?.min_age.toString(),
    maxAge: currentUserProfile?.max_age.toString(),
    minRating: '',
    maxRating: '',
    multipleTags: [] as ProfileTag[]
  })

  const [activeFilters, setActiveFilters] = useState(filters)

  const handleToggleCommonTag = (tag: ProfileTag) => {
    setFilters((prev) => {
      const alreadySelected = prev.multipleTags.some((t) => t.id === tag.id)
      const newTags = alreadySelected
        ? prev.multipleTags.filter((t) => t.id !== tag.id)
        : [...prev.multipleTags, tag]
      return { ...prev, multipleTags: newTags }
    })
  }

  const handleToggleMultipleTag = (tag: ProfileTag) => {
    setFilters((prev) => {
      const alreadySelected = prev.multipleTags.some((t) => t.id === tag.id)
      const newTags = alreadySelected
        ? prev.multipleTags.filter((t) => t.id !== tag.id)
        : [...prev.multipleTags, tag]
      return { ...prev, multipleTags: newTags }
    })
  }

  const handleAdvancedSearchToggle = () => {
    setFilters({
      age: '',
      radius_km: '300',
      rating: '',
      minAge: '',
      maxAge: '',
      minRating: '',
      maxRating: '',
      multipleTags: []
    })
    setIsAdvancedSearch((prev) => !prev)
  }

  const {
    data,
    isLoading,
    isError,
    isFetchingNextPage,
    fetchNextPage,
    hasNextPage,
    refetch
  } = useInfiniteQuery({
    queryKey: ['users', activeFilters, sortOption],
    queryFn: async ({ pageParam = 0 }) => {
      const query: Record<string, any> = {
        offset: pageParam,
        limit: 25,
        sort_by: sortOption,
        sort_order: SortOrder.Asc,
        ...(activeFilters.rating && {
          min_fame_rating: parseInt(activeFilters.rating),
          max_fame_rating: parseInt(activeFilters.rating)
        }),
        ...(activeFilters.radius_km && {
          radius_km: parseInt(activeFilters.radius_km)
        }),
        ...(activeFilters.multipleTags.length > 0 && {
          tag_ids: activeFilters.multipleTags.map((t) => t.id)
        }),
        ...(activeFilters.minAge && {
          min_age: parseInt(activeFilters.minAge)
        }),
        ...(activeFilters.maxAge && {
          max_age: parseInt(activeFilters.maxAge)
        }),
        ...(activeFilters.minRating && {
          min_fame_rating: parseInt(activeFilters.minRating)
        }),
        ...(activeFilters.maxRating && {
          max_fame_rating: parseInt(activeFilters.maxRating)
        })
      }

      const response = await api.v1.searchProfile(query)
      return response.data
    },
    initialPageParam: 0,
    getNextPageParam: (lastPage, allPages) => {
      if (lastPage.length < 25) {
        return undefined
      }
      const totalFetchedSoFar = allPages.reduce(
        (acc, page) => acc + page.length,
        0
      )
      return totalFetchedSoFar
    }
  })

  useEffect(() => {
    refetch()
  }, [sortOption, activeFilters, refetch])

  const applyFilters = () => {
    setActiveFilters(filters)
    setIsDialogOpen(false)
    console.log('Filters applied:', filters, 'Sorting applied:', sortOption)
  }

  const allUsers: UserProfile[] = data?.pages.flatMap((page) => page) || []

  return (
    <div className="container w-full px-0">
      <div className="mb-4 flex justify-between">
        <Select
          onValueChange={(value) => {
            setSortOption(value as UserProfileSortBy)
          }}
          defaultValue={UserProfileSortBy.Distance}
        >
          <SelectTrigger className="w-fit gap-3 bg-white">
            <SelectValue placeholder="Sort by" />
          </SelectTrigger>
          <SelectContent className="bg-white">
            <SelectItem value={UserProfileSortBy.Distance}>Distance</SelectItem>
            <SelectItem value={UserProfileSortBy.Age}>Age</SelectItem>
            <SelectItem value={UserProfileSortBy.FameRating}>Rating</SelectItem>
            <SelectItem value={UserProfileSortBy.Tags}>Tags</SelectItem>
          </SelectContent>
        </Select>

        <Dialog open={isDialogOpen} onOpenChange={setIsDialogOpen}>
          <DialogTrigger asChild>
            <Button variant="outline" onClick={() => setIsDialogOpen(true)}>
              <ListFilter /> Filter
            </Button>
          </DialogTrigger>

          <DialogContent className="max-w-md">
            <DialogHeader>
              <DialogTitle>Filter Users</DialogTitle>
              <DialogDescription className="text-xs">
                {isAdvancedSearch
                  ? 'Advanced criteria: set ranges and multiple tags.'
                  : 'Basic filters: single age, radius, etc.'}
              </DialogDescription>
            </DialogHeader>

            {!isAdvancedSearch && (
              <div className="space-y-4">
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

                <div>
                  <p className="mb-2 text-sm font-medium">Rating</p>
                  <Slider
                    value={[parseInt(filters.rating) || 0]}
                    min={0}
                    max={100}
                    step={1}
                    onValueChange={(val) => {
                      setFilters((prev) => ({
                        ...prev,
                        rating: val[0].toString()
                      }))
                    }}
                    className={cn('w-full')}
                  />
                  <p className="mt-1 text-xs">
                    Selected Rating: {filters.rating || '0'}
                  </p>
                </div>

                <div>
                  <p className="mb-2 text-sm font-medium">Distance (km)</p>
                  <Slider
                    value={[parseInt(filters.radius_km) || 300]}
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

                <div>
                  <p className="mb-2 text-sm font-medium">Multiple Tags</p>
                  <TagSelector
                    selectedTags={filters.multipleTags}
                    onToggleTag={handleToggleCommonTag}
                  />
                  <div className="mt-2 flex flex-wrap gap-2">
                    {filters.multipleTags.map((tag) => (
                      <span
                        key={tag.id}
                        className="inline-flex cursor-pointer items-center rounded-full border border-white/20 bg-black/80 px-2 py-1 text-[8px] font-normal text-white"
                      >
                        {tag.name.split('_')[0]}
                      </span>
                    ))}
                  </div>
                </div>
              </div>
            )}

            {isAdvancedSearch && (
              <div className="space-y-4">
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

                <div>
                  <p className="mb-2 text-sm font-medium">Rating Range</p>
                  <Slider
                    value={[
                      parseInt(filters.minRating) || 0,
                      parseInt(filters.maxRating) || 100
                    ]}
                    min={0}
                    max={100}
                    step={1}
                    onValueChange={(vals) => {
                      setFilters((prev) => ({
                        ...prev,
                        minRating: vals[0].toString(),
                        maxRating: vals[1].toString()
                      }))
                    }}
                    className={cn('w-full')}
                  />
                  <p className="mt-1 text-xs">
                    {filters.minRating || 0} - {filters.maxRating || 100}
                  </p>
                </div>

                <div>
                  <p className="mb-2 text-sm font-medium">Distance (km)</p>
                  <Slider
                    value={[parseInt(filters.radius_km) || 300]}
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

                <div>
                  <p className="mb-2 text-sm font-medium">Multiple Tags</p>
                  <TagSelector
                    selectedTags={filters.multipleTags}
                    onToggleTag={handleToggleMultipleTag}
                  />
                  <div className="mt-2 flex flex-wrap gap-2">
                    {filters.multipleTags.map((tag) => (
                      <span
                        key={tag.id}
                        className="inline-flex cursor-pointer items-center rounded-full border border-white/20 bg-black/80 px-2 py-1 text-[8px] font-normal text-white"
                      >
                        {tag.name.split('_')[0]}
                      </span>
                    ))}
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

      {isLoading ? (
        <div>Loading...</div>
      ) : isError ? (
        <div>Something went wrong. Please try again.</div>
      ) : (
        <>
          <div className="flex w-full flex-wrap justify-start gap-6">
            {allUsers.map((user) => (
              <UserCard key={user.id} user={user} />
            ))}
          </div>

          {hasNextPage && (
            <div className="mt-6 flex justify-center">
              <Button
                onClick={() => fetchNextPage()}
                disabled={isFetchingNextPage}
              >
                {isFetchingNextPage ? 'Loading more...' : 'Load More'}
              </Button>
            </div>
          )}
        </>
      )}
    </div>
  )
}

function UserCard({ user }: { user: UserProfile }) {
  const [isConfettiActive, setIsConfettiActive] = useState(false)
  const { toast } = useToast()

  const likeMutation = useMutation({
    mutationFn: async () => {
      return api.v1.likeUserProfile(user.id)
    },
    onSuccess: () => {
      setIsConfettiActive(true)
      setTimeout(() => {
        setIsConfettiActive(false)
      }, 1000)
      if (user.meta) {
        user.meta.is_liked = true
      }
      toast({
        title: 'Liked',
        description: `You have liked ${user?.name}'s profile.`
      })
    },
    onError: (err: any) => {
      toast({
        title: 'Action failed',
        description:
          err.message || 'Something went wrong while liking the profile.',
        variant: 'destructive'
      })
    }
  })

  const unlikeMutation = useMutation({
    mutationFn: async () => {
      return api.v1.removeUserProfileLike(user.id)
    },
    onSuccess: () => {
      if (user.meta) {
        user.meta.is_liked = false
      }
      toast({
        title: 'Unliked',
        description: `You have unliked ${user?.name}'s profile.`
      })
    },
    onError: (err: any) => {
      toast({
        title: 'Action failed',
        description:
          err.message || 'Something went wrong while unliking the profile.',
        variant: 'destructive'
      })
    }
  })

  const handleLikeClick = (e: React.MouseEvent<HTMLButtonElement>) => {
    e.preventDefault()
    e.stopPropagation()
    if (user?.meta?.is_liked) {
      unlikeMutation.mutate()
    } else {
      likeMutation.mutate()
    }
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
                <p className="text-[8px] font-light">{user.rating}</p>
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
            {user?.meta?.is_liked ? (
              <FaHeartCrack color="white" />
            ) : (
              <FaHeart color="white" />
            )}
          </Button>
        </div>

        <div className="mt-2 flex flex-wrap gap-1 px-4 pb-4">
          {user.tags.slice(0, 3).map((tag) => (
            <Badge
              key={tag.id}
              variant="secondary"
              className="rounded-full border border-white/20 bg-black/80 py-1 text-[6px] font-normal text-white hover:bg-black/80"
            >
              {tag.name.split('_')[0]}
            </Badge>
          ))}
        </div>
      </div>
    </Link>
  )
}

export default Search
