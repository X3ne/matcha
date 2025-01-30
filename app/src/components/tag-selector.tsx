'use client'
import api, { type ProfileTag } from '@/api'
import { Button } from '@/components/ui/button'
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogTrigger
} from '@/components/ui/dialog'
import { ScrollArea } from '@/components/ui/scroll-area'
import { useQuery } from '@tanstack/react-query'

interface TagSelectorProps {
  selectedTags: ProfileTag[]
  onToggleTag: (tag: ProfileTag) => void
}

export default function TagSelector({
  selectedTags,
  onToggleTag
}: TagSelectorProps) {
  const {
    data: tags,
    isLoading,
    error
  } = useQuery({
    queryKey: ['tags'],
    queryFn: async () => (await api.v1.getAllTags()).data as ProfileTag[]
  })

  if (isLoading) return <p>Loading tags...</p>
  if (error) return <p>Failed to load tags</p>

  return (
    <Dialog>
      <DialogTrigger asChild>
        <Button size={'sm'} variant="outline" className="w-full">
          Select Interests
        </Button>
      </DialogTrigger>
      <DialogContent className="sm:max-w-xl">
        <DialogHeader>
          <DialogTitle>Select Your Interests</DialogTitle>
        </DialogHeader>
        <ScrollArea className="mt-4 max-h-[60vh]">
          <div className="flex flex-wrap gap-2 p-4">
            {tags?.map((tag) => (
              <div
                key={tag.id}
                className={`inline-flex cursor-pointer items-center rounded-full border px-5 py-2 text-xs font-normal ${
                  selectedTags.some((t) => t.id === tag.id)
                    ? 'border-white/20 bg-black/80 text-white'
                    : 'border-black/20 bg-white text-black'
                }`}
                onClick={() => onToggleTag(tag)}
              >
                {tag.name}
              </div>
            ))}
          </div>
        </ScrollArea>
      </DialogContent>
    </Dialog>
  )
}
