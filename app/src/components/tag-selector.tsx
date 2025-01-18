'use client'

import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogTrigger
} from '@/components/ui/dialog'
import { ScrollArea } from '@/components/ui/scroll-area'

const availableTags = [
  'vegan',
  'vegetarian',
  'geek',
  'nerd',
  'artist',
  'musician',
  'writer',
  'traveler',
  'foodie',
  'fitness',
  'yoga',
  'meditation',
  'outdoor',
  'adventure',
  'photography',
  'fashion',
  'design',
  'technology',
  'science',
  'history',
  'politics',
  'philosophy',
  'sports',
  'gaming',
  'anime',
  'movies',
  'books',
  'cooking',
  'dancing',
  'singing'
]

interface TagSelectorProps {
  selectedTags: string[]
  onToggleTag: (tag: string) => void
}

export default function TagSelector({
  selectedTags,
  onToggleTag
}: TagSelectorProps) {
  return (
    <Dialog>
      <DialogTrigger asChild>
        <Button variant="outline" className="w-full">
          Select Interests
        </Button>
      </DialogTrigger>
      <DialogContent className="sm:max-w-xl">
        <DialogHeader>
          <DialogTitle>Select Your Interests</DialogTitle>
        </DialogHeader>
        <ScrollArea className="mt-4 max-h-[60vh]">
          <div className="flex flex-wrap gap-2 p-4">
            {availableTags.map((tag) => (
              <div
                key={tag}
                className={`inline-flex cursor-pointer items-center rounded-full border px-5 py-2 text-xs font-normal ${
                  selectedTags.includes(tag)
                    ? 'border-white/20 bg-black/80 text-white'
                    : 'border-black/20 bg-white text-black'
                }`}
                onClick={() => onToggleTag(tag)}
              >
                #{tag}
              </div>
            ))}
          </div>
        </ScrollArea>
      </DialogContent>
    </Dialog>
  )
}
