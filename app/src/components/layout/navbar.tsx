'use client'

import { NotificationDropdown } from '@/components/notification-dropdown'
import { Button } from '@/components/ui/button'
import {
  Sheet,
  SheetContent,
  SheetHeader,
  SheetTitle,
  SheetTrigger
} from '@/components/ui/sheet'
import { UserContext } from '@/providers/userProvider'
import { Link } from '@tanstack/react-router'
import { useNavigate } from '@tanstack/react-router'
import { Mail, Bell, Menu } from 'lucide-react'
import { useState } from 'react'
import { useContext } from 'react'
import { FaRegHeart } from 'react-icons/fa'

const Navbar = () => {
  const navigate = useNavigate()

  const { user, logout } = useContext(UserContext)
  const [isSheetOpen, setIsSheetOpen] = useState(false)

  const handleCloseSheet = () => {
    setIsSheetOpen(false) // Close the sheet
  }

  const desktopLoggedInLinks = (
    <ul className="hidden gap-8 md:flex">
      <li>
        <Link to="/search">Search</Link>
      </li>
      <li>
        <Link to="/profile">Profile</Link>
      </li>
    </ul>
  )

  const desktopGuestLinks = (
    <ul className="hidden gap-4 md:flex">
      <li>
        <Link to="/login">
          <Button>Login</Button>
        </Link>
      </li>
      <li>
        <Link to="/register">
          <Button>Register</Button>
        </Link>
      </li>
    </ul>
  )

  const mobileLoggedInLinks = (
    <>
      <Button
        asChild
        variant="outline"
        className="w-full"
        onClick={handleCloseSheet}
      >
        <Link to="/search">Search</Link>
      </Button>
      <Button
        asChild
        variant="outline"
        className="w-full"
        onClick={handleCloseSheet}
      >
        <Link to="/profile">Profile</Link>
      </Button>
    </>
  )

  const mobileGuestLinks = (
    <>
      <Button asChild className="w-full" onClick={handleCloseSheet}>
        <Link to="/login">Login</Link>
      </Button>
      <Button asChild className="w-full" onClick={handleCloseSheet}>
        <Link to="/register">Register</Link>
      </Button>
    </>
  )

  const handleMailClick = () => {
    navigate({ to: '/messages' })
  }

  return (
    <nav className="bg-white">
      <div className="mx-auto flex max-w-screen-xl items-center justify-between px-8 py-4 sm:px-6">
        <div className="flex gap-2">
          <FaRegHeart className="self-center text-primary" size={23} />
          <Link to="/search" className="text-2xl font-semibold">
            Matcha
          </Link>
        </div>

        {user ? desktopLoggedInLinks : desktopGuestLinks}

        {user && (
          <div className="hidden gap-4 md:flex">
            <div className="flex gap-0.5">
              <Button
                variant="ghost"
                size="icon"
                className="relative"
                aria-label="Messages"
                onClick={handleMailClick}
              >
                <Mail className="!size-5" />
              </Button>
              <NotificationDropdown />
            </div>
            <Button onClick={logout}>Logout</Button>
          </div>
        )}

        <div className="md:hidden">
          <Sheet open={isSheetOpen} onOpenChange={setIsSheetOpen}>
            <SheetTrigger asChild>
              <Button
                className="rounded hover:bg-gray-200"
                aria-label="Open Menu"
              >
                <Menu size={20} />
              </Button>
            </SheetTrigger>

            <SheetContent side="right">
              <SheetHeader>
                <SheetTitle>Matcha</SheetTitle>
              </SheetHeader>

              <div className="mt-8 flex h-full flex-col gap-4">
                {user ? mobileLoggedInLinks : mobileGuestLinks}

                {user && (
                  <div className="mb-10 mt-auto flex flex-col gap-4">
                    <Button
                      className="relative rounded p-2 hover:bg-gray-200"
                      aria-label="Messages"
                      onClick={handleMailClick}
                    >
                      <Mail size={20} />
                    </Button>
                    <Button
                      className="relative p-2 text-left hover:bg-gray-200"
                      aria-label="Notifications"
                      onClick={handleCloseSheet}
                    >
                      <Bell size={20} />
                    </Button>
                    <Button
                      onClick={() => {
                        logout()
                        handleCloseSheet()
                      }}
                      className="w-full"
                    >
                      Logout
                    </Button>
                  </div>
                )}
              </div>
            </SheetContent>
          </Sheet>
        </div>
      </div>
    </nav>
  )
}

export default Navbar
