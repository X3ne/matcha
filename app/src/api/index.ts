import { Api } from './spec'

export default new Api({
  baseUrl: import.meta.env.VITE_API_URL
})
export * from './spec'
