import { useQueryClient } from '@tanstack/react-query'
import { createLazyFileRoute } from '@tanstack/react-router'
import { useEffect, useState } from 'react'

export const Route = createLazyFileRoute('/error/')({
  component: Error
})

function Error() {
  const queryClient = useQueryClient()
  const max_retries = 100
  const [retry, setRetry] = useState(0)

  useEffect(() => {
    if (retry >= max_retries) return
    const interval = setInterval(() => {
      queryClient.invalidateQueries({
        queryKey: ['health']
      })
      setRetry((prev) => prev + 1)
    }, 2000)

    return () => {
      clearInterval(interval)
    }
  }, [])

  return (
    <div className="flex items-center justify-center h-screen">
      <div className="text-center">
        <h1 className="text-5xl font-bold text-foreground">Error</h1>
        <p className="text-foreground">An error occurred with the server.</p>
        <p className="text-foreground">
          {retry}/{max_retries}
        </p>
      </div>
    </div>
  )
}
