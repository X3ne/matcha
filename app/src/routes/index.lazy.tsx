import { createLazyFileRoute, useNavigate } from '@tanstack/react-router'

export const Route = createLazyFileRoute('/')({
  component: Index
})

function Index() {
  const navigation = useNavigate()

  navigation({ to: '/search' })
  return <></>
}
