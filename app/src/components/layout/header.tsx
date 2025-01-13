import { Button } from '@/components/ui/button'

export function Header() {
  return (
    <header className="py-4 px-8 flex justify-between">
      <h1 className="text-4xl font-semibold">Matcha</h1>
      <Button variant="outline">Login</Button>
    </header>
  )
}
