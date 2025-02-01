import { LoginForm } from '@/components/login-form'
import { createLazyFileRoute } from '@tanstack/react-router'
import { FaRegHeart } from 'react-icons/fa'

export const Route = createLazyFileRoute('/login/')({
  component: RouteComponent
})

function RouteComponent() {
  return (
    <div className="motion-preset-slide-down motion-duration-500 flex flex-col items-center justify-center gap-6">
      <div className="flex w-full max-w-sm flex-col gap-6">
        <a href="#" className="flex items-center gap-2 self-center font-medium">
          <div className="flex h-6 w-6 items-center justify-center rounded-md bg-primary text-primary-foreground">
            <FaRegHeart color="white" />
          </div>
          Matcha.
        </a>
        <LoginForm />
      </div>
    </div>
  )
}
