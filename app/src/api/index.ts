import { Api } from './spec'

export default new Api({
  baseUrl: import.meta.env.VITE_API_URL,
  baseApiParams: {
    headers: {
      'Content-Type': 'application/json',
      Accept: 'application/json'
    }
  }
})
export * from './spec'
