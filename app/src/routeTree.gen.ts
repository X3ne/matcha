/* eslint-disable */

// @ts-nocheck

// noinspection JSUnusedGlobalSymbols

// This file was automatically generated by TanStack Router.
// You should NOT make any changes in this file as it will be overwritten.
// Additionally, you should also exclude this file from your linter and/or formatter to prevent it from being checked or modified.

import { createFileRoute } from '@tanstack/react-router'

// Import Routes

import { Route as rootRoute } from './routes/__root'

// Create Virtual Routes

const IndexLazyImport = createFileRoute('/')()
const SearchIndexLazyImport = createFileRoute('/search/')()
const RegisterIndexLazyImport = createFileRoute('/register/')()
const ProfileIndexLazyImport = createFileRoute('/profile/')()
const OnboardingIndexLazyImport = createFileRoute('/onboarding/')()
const MessagesIndexLazyImport = createFileRoute('/messages/')()
const LoginIndexLazyImport = createFileRoute('/login/')()
const ErrorIndexLazyImport = createFileRoute('/error/')()
const ProfileIdLazyImport = createFileRoute('/profile/$id')()

// Create/Update Routes

const IndexLazyRoute = IndexLazyImport.update({
  id: '/',
  path: '/',
  getParentRoute: () => rootRoute,
} as any).lazy(() => import('./routes/index.lazy').then((d) => d.Route))

const SearchIndexLazyRoute = SearchIndexLazyImport.update({
  id: '/search/',
  path: '/search/',
  getParentRoute: () => rootRoute,
} as any).lazy(() => import('./routes/search/index.lazy').then((d) => d.Route))

const RegisterIndexLazyRoute = RegisterIndexLazyImport.update({
  id: '/register/',
  path: '/register/',
  getParentRoute: () => rootRoute,
} as any).lazy(() =>
  import('./routes/register/index.lazy').then((d) => d.Route),
)

const ProfileIndexLazyRoute = ProfileIndexLazyImport.update({
  id: '/profile/',
  path: '/profile/',
  getParentRoute: () => rootRoute,
} as any).lazy(() => import('./routes/profile/index.lazy').then((d) => d.Route))

const OnboardingIndexLazyRoute = OnboardingIndexLazyImport.update({
  id: '/onboarding/',
  path: '/onboarding/',
  getParentRoute: () => rootRoute,
} as any).lazy(() =>
  import('./routes/onboarding/index.lazy').then((d) => d.Route),
)

const MessagesIndexLazyRoute = MessagesIndexLazyImport.update({
  id: '/messages/',
  path: '/messages/',
  getParentRoute: () => rootRoute,
} as any).lazy(() =>
  import('./routes/messages/index.lazy').then((d) => d.Route),
)

const LoginIndexLazyRoute = LoginIndexLazyImport.update({
  id: '/login/',
  path: '/login/',
  getParentRoute: () => rootRoute,
} as any).lazy(() => import('./routes/login/index.lazy').then((d) => d.Route))

const ErrorIndexLazyRoute = ErrorIndexLazyImport.update({
  id: '/error/',
  path: '/error/',
  getParentRoute: () => rootRoute,
} as any).lazy(() => import('./routes/error/index.lazy').then((d) => d.Route))

const ProfileIdLazyRoute = ProfileIdLazyImport.update({
  id: '/profile/$id',
  path: '/profile/$id',
  getParentRoute: () => rootRoute,
} as any).lazy(() => import('./routes/profile/$id.lazy').then((d) => d.Route))

// Populate the FileRoutesByPath interface

declare module '@tanstack/react-router' {
  interface FileRoutesByPath {
    '/': {
      id: '/'
      path: '/'
      fullPath: '/'
      preLoaderRoute: typeof IndexLazyImport
      parentRoute: typeof rootRoute
    }
    '/profile/$id': {
      id: '/profile/$id'
      path: '/profile/$id'
      fullPath: '/profile/$id'
      preLoaderRoute: typeof ProfileIdLazyImport
      parentRoute: typeof rootRoute
    }
    '/error/': {
      id: '/error/'
      path: '/error'
      fullPath: '/error'
      preLoaderRoute: typeof ErrorIndexLazyImport
      parentRoute: typeof rootRoute
    }
    '/login/': {
      id: '/login/'
      path: '/login'
      fullPath: '/login'
      preLoaderRoute: typeof LoginIndexLazyImport
      parentRoute: typeof rootRoute
    }
    '/messages/': {
      id: '/messages/'
      path: '/messages'
      fullPath: '/messages'
      preLoaderRoute: typeof MessagesIndexLazyImport
      parentRoute: typeof rootRoute
    }
    '/onboarding/': {
      id: '/onboarding/'
      path: '/onboarding'
      fullPath: '/onboarding'
      preLoaderRoute: typeof OnboardingIndexLazyImport
      parentRoute: typeof rootRoute
    }
    '/profile/': {
      id: '/profile/'
      path: '/profile'
      fullPath: '/profile'
      preLoaderRoute: typeof ProfileIndexLazyImport
      parentRoute: typeof rootRoute
    }
    '/register/': {
      id: '/register/'
      path: '/register'
      fullPath: '/register'
      preLoaderRoute: typeof RegisterIndexLazyImport
      parentRoute: typeof rootRoute
    }
    '/search/': {
      id: '/search/'
      path: '/search'
      fullPath: '/search'
      preLoaderRoute: typeof SearchIndexLazyImport
      parentRoute: typeof rootRoute
    }
  }
}

// Create and export the route tree

export interface FileRoutesByFullPath {
  '/': typeof IndexLazyRoute
  '/profile/$id': typeof ProfileIdLazyRoute
  '/error': typeof ErrorIndexLazyRoute
  '/login': typeof LoginIndexLazyRoute
  '/messages': typeof MessagesIndexLazyRoute
  '/onboarding': typeof OnboardingIndexLazyRoute
  '/profile': typeof ProfileIndexLazyRoute
  '/register': typeof RegisterIndexLazyRoute
  '/search': typeof SearchIndexLazyRoute
}

export interface FileRoutesByTo {
  '/': typeof IndexLazyRoute
  '/profile/$id': typeof ProfileIdLazyRoute
  '/error': typeof ErrorIndexLazyRoute
  '/login': typeof LoginIndexLazyRoute
  '/messages': typeof MessagesIndexLazyRoute
  '/onboarding': typeof OnboardingIndexLazyRoute
  '/profile': typeof ProfileIndexLazyRoute
  '/register': typeof RegisterIndexLazyRoute
  '/search': typeof SearchIndexLazyRoute
}

export interface FileRoutesById {
  __root__: typeof rootRoute
  '/': typeof IndexLazyRoute
  '/profile/$id': typeof ProfileIdLazyRoute
  '/error/': typeof ErrorIndexLazyRoute
  '/login/': typeof LoginIndexLazyRoute
  '/messages/': typeof MessagesIndexLazyRoute
  '/onboarding/': typeof OnboardingIndexLazyRoute
  '/profile/': typeof ProfileIndexLazyRoute
  '/register/': typeof RegisterIndexLazyRoute
  '/search/': typeof SearchIndexLazyRoute
}

export interface FileRouteTypes {
  fileRoutesByFullPath: FileRoutesByFullPath
  fullPaths:
    | '/'
    | '/profile/$id'
    | '/error'
    | '/login'
    | '/messages'
    | '/onboarding'
    | '/profile'
    | '/register'
    | '/search'
  fileRoutesByTo: FileRoutesByTo
  to:
    | '/'
    | '/profile/$id'
    | '/error'
    | '/login'
    | '/messages'
    | '/onboarding'
    | '/profile'
    | '/register'
    | '/search'
  id:
    | '__root__'
    | '/'
    | '/profile/$id'
    | '/error/'
    | '/login/'
    | '/messages/'
    | '/onboarding/'
    | '/profile/'
    | '/register/'
    | '/search/'
  fileRoutesById: FileRoutesById
}

export interface RootRouteChildren {
  IndexLazyRoute: typeof IndexLazyRoute
  ProfileIdLazyRoute: typeof ProfileIdLazyRoute
  ErrorIndexLazyRoute: typeof ErrorIndexLazyRoute
  LoginIndexLazyRoute: typeof LoginIndexLazyRoute
  MessagesIndexLazyRoute: typeof MessagesIndexLazyRoute
  OnboardingIndexLazyRoute: typeof OnboardingIndexLazyRoute
  ProfileIndexLazyRoute: typeof ProfileIndexLazyRoute
  RegisterIndexLazyRoute: typeof RegisterIndexLazyRoute
  SearchIndexLazyRoute: typeof SearchIndexLazyRoute
}

const rootRouteChildren: RootRouteChildren = {
  IndexLazyRoute: IndexLazyRoute,
  ProfileIdLazyRoute: ProfileIdLazyRoute,
  ErrorIndexLazyRoute: ErrorIndexLazyRoute,
  LoginIndexLazyRoute: LoginIndexLazyRoute,
  MessagesIndexLazyRoute: MessagesIndexLazyRoute,
  OnboardingIndexLazyRoute: OnboardingIndexLazyRoute,
  ProfileIndexLazyRoute: ProfileIndexLazyRoute,
  RegisterIndexLazyRoute: RegisterIndexLazyRoute,
  SearchIndexLazyRoute: SearchIndexLazyRoute,
}

export const routeTree = rootRoute
  ._addFileChildren(rootRouteChildren)
  ._addFileTypes<FileRouteTypes>()

/* ROUTE_MANIFEST_START
{
  "routes": {
    "__root__": {
      "filePath": "__root.tsx",
      "children": [
        "/",
        "/profile/$id",
        "/error/",
        "/login/",
        "/messages/",
        "/onboarding/",
        "/profile/",
        "/register/",
        "/search/"
      ]
    },
    "/": {
      "filePath": "index.lazy.tsx"
    },
    "/profile/$id": {
      "filePath": "profile/$id.lazy.tsx"
    },
    "/error/": {
      "filePath": "error/index.lazy.tsx"
    },
    "/login/": {
      "filePath": "login/index.lazy.tsx"
    },
    "/messages/": {
      "filePath": "messages/index.lazy.tsx"
    },
    "/onboarding/": {
      "filePath": "onboarding/index.lazy.tsx"
    },
    "/profile/": {
      "filePath": "profile/index.lazy.tsx"
    },
    "/register/": {
      "filePath": "register/index.lazy.tsx"
    },
    "/search/": {
      "filePath": "search/index.lazy.tsx"
    }
  }
}
ROUTE_MANIFEST_END */
