import { createLazyFileRoute, Link } from '@tanstack/react-router'

export const Route = createLazyFileRoute('/')({
  component: Index
})

function Index() {
  return (
    <>
      <div className="relative h-dvh">
        <h1 className="text-4xl font-bold">Index</h1>
      </div>
    </>
  )
}
