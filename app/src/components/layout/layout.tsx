import React from 'react'

import { Footer } from './footer'
import Navbar from './navbar'

type LayoutProps = {
  children: React.ReactNode
}

export function Layout({ children }: LayoutProps) {
  return (
    <div className="flex min-h-screen flex-col">
      <Navbar />
      <main className="flex-1 py-16">
        <div className="mx-auto w-full max-w-screen-xl flex-1 px-8 sm:px-6">
          {children}
        </div>
      </main>
      <Footer />
    </div>
  )
}
