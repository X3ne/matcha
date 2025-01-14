import { createLazyFileRoute } from '@tanstack/react-router'

export const Route = createLazyFileRoute('/profile/$username')({
  component: RouteComponent
})

function RouteComponent() {
  return <div>Hello "/profile/$username"!</div>
}
