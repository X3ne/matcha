'use client'

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
import { useState } from 'react'
import { useContext } from 'react'
import { FaBars, FaBell } from 'react-icons/fa6'

const Navbar = () => {
  const { user, logout } = useContext(UserContext)
  const [isSheetOpen, setIsSheetOpen] = useState(false) // Sheet open state

  const handleCloseSheet = () => {
    setIsSheetOpen(false) // Close the sheet
  }

  // Desktop links (only for screens md and up)
  const desktopLoggedInLinks = (
    <ul className="hidden gap-8 md:flex">
      <li>
        <Link to="/message">Message</Link>
      </li>
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
        <Button>
          <Link to="/login">Login</Link>
        </Button>
      </li>
      <li>
        <Button>
          <Link to="/register">Register</Link>
        </Button>
      </li>
    </ul>
  )

  // Mobile (Sheet) links are turned into full-width Buttons
  const mobileLoggedInLinks = (
    <>
      <Button
        asChild
        variant="outline"
        className="w-full"
        onClick={handleCloseSheet}
      >
        <Link to="/message">Message</Link>
      </Button>
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

  return (
    <nav className="bg-white">
      <div className="mx-auto flex max-w-screen-xl items-center justify-between px-8 py-4 sm:px-6">
        {/* Logo */}
        <Link to="/" className="text-3xl font-semibold">
          Matcha
        </Link>

        {user ? desktopLoggedInLinks : desktopGuestLinks}

        {user && (
          <div className="hidden gap-4 md:flex">
            <button
              className="relative rounded-full p-2 hover:bg-gray-200"
              aria-label="Notifications"
            >
              <FaBell size={20} />
            </button>
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
                <FaBars size={20} />
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
                      className="relative p-2 text-left hover:bg-gray-200"
                      aria-label="Notifications"
                      onClick={handleCloseSheet}
                    >
                      <FaBell size={20} />
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
