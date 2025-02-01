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
      <main className="flex w-full flex-1 bg-muted py-10">
        <div className="mx-auto flex w-full max-w-screen-xl px-8 sm:px-6">
          {children}
        </div>
      </main>
      <Footer />
    </div>
  )
}
