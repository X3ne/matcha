/* eslint-disable */
/* tslint:disable */
/*
 * ---------------------------------------------------------------
 * ## THIS FILE WAS GENERATED VIA SWAGGER-TYPESCRIPT-API        ##
 * ##                                                           ##
 * ## AUTHOR: acacode                                           ##
 * ## SOURCE: https://github.com/acacode/swagger-typescript-api ##
 * ---------------------------------------------------------------
 */

/** Channel */
export interface Channel {
  /** A 64 bit integer unique identifier (serialized as string to avoid overflow issues) */
  id: Snowflake
  name: string
  participants: ChannelParticipant[]
}

export interface ChannelParticipant {
  avatar?: string | null
  /** A 64 bit integer unique identifier (serialized as string to avoid overflow issues) */
  id: Snowflake
  name: string
}

/** CompleteOnboarding */
export interface CompleteOnboarding {
  /**
   * @format uint
   * @min 0
   * @default 0
   */
  avatar_index?: number
  bio?: string | null
  /** @format date */
  birth_date: string
  gender: Gender
  location?: Location | null
  /** @format int32 */
  max_age: number
  /** @format int32 */
  max_distance_km: number
  /** @format int32 */
  min_age: number
  name: string
  /** @default "bisexual" */
  sexual_orientation?: Orientation
}

export interface CompleteOnboardingForm {
  /** @format binary */
  pictures?: string
  profile?: CompleteOnboarding
}

export enum Gender {
  Male = 'male',
  Female = 'female'
}

export interface Location {
  /** @format double */
  latitude: number
  /** @format double */
  longitude: number
}

/** Login */
export interface Login {
  password: string
  username: string
}

/** Message */
export interface Message {
  author: MessageAuthor
  /** A 64 bit integer unique identifier (serialized as string to avoid overflow issues) */
  channel_id: Snowflake
  content: string
  /** @format partial-date-time */
  edited_at?: string | null
  /** A 64 bit integer unique identifier (serialized as string to avoid overflow issues) */
  id: Snowflake
  /** @format partial-date-time */
  sent_at: string
}

export interface MessageAuthor {
  avatar?: string | null
  /** A 64 bit integer unique identifier (serialized as string to avoid overflow issues) */
  id: Snowflake
  name: string
}

export enum MessageSortBy {
  SentAt = 'sent_at'
}

/** OAuthResponse */
export interface OAuthResponse {
  url: string
}

export enum Orientation {
  Male = 'male',
  Female = 'female',
  Bisexual = 'bisexual'
}

/** PartialUserProfile */
export interface PartialUserProfile {
  /** @format int32 */
  age: number
  /**
   * @format uint64
   * @min 0
   */
  approx_distance_km?: number | null
  avatar_url?: string | null
  bio?: string | null
  gender: Gender
  /** A 64 bit integer unique identifier (serialized as string to avoid overflow issues) */
  id: Snowflake
  meta?: UserProfileMeta | null
  name: string
  picture_urls: string[]
  /** @format int32 */
  rating: number
  sexual_orientation: Orientation
  tags: ProfileTag[]
}

/** PostMessage */
export interface PostMessage {
  content: string
}

/** ProfileTag */
export interface ProfileTag {
  /** A 64 bit integer unique identifier (serialized as string to avoid overflow issues) */
  id: Snowflake
  name: string
}

/** RegisterUser */
export interface RegisterUser {
  confirm_password: string
  email: string
  first_name: string
  last_name: string
  password: string
  username: string
}

/** ReportProfile */
export interface ReportProfile {
  /** @default false */
  block_user?: boolean
  reason?: string | null
}

/** RequestResetPassword */
export interface RequestResetPassword {
  email: string
}

/** ResetPassword */
export interface ResetPassword {
  confirm_password: string
  email: string
  password: string
  token: string
}

/** ServerHealth */
export interface ServerHealth {
  status: string
}

/**
 * A 64 bit integer unique identifier (serialized as string to avoid overflow issues)
 * @format snowflake
 * @example "1869760527605956608"
 */
export type Snowflake = string

export enum SortOrder {
  Asc = 'asc',
  Desc = 'desc'
}

/** UpdateProfile */
export interface UpdateProfile {
  bio?: string | null
  gender?: Gender | null
  location?: Location | null
  /** @format int32 */
  max_age: number
  /** @format int32 */
  max_distance_km: number
  /** @format int32 */
  min_age: number
  name?: string | null
  sexual_orientation?: Orientation | null
}

/** UpdateUser */
export interface UpdateUser {
  email?: string | null
  first_name?: string | null
  last_name?: string | null
  username?: string | null
}

export interface UploadProfilePictureForm {
  /** @format binary */
  picture?: File
}

/** User */
export interface User {
  email: string
  first_name: string
  /** A 64 bit integer unique identifier (serialized as string to avoid overflow issues) */
  id: Snowflake
  last_name: string
  username: string
}

/** UserProfile */
export interface UserProfile {
  /** @format int32 */
  age: number
  avatar_url?: string | null
  bio?: string | null
  gender: Gender
  /** A 64 bit integer unique identifier (serialized as string to avoid overflow issues) */
  id: Snowflake
  /**
   * @format uint8
   * @min 0
   */
  max_age: number
  /** @format int32 */
  max_distance_km: number
  /**
   * @format uint8
   * @min 0
   */
  min_age: number
  name: string
  picture_urls: string[]
  /** @format int32 */
  rating: number
  sexual_orientation: Orientation
  tags: ProfileTag[]
}

/** UserProfileBulkTags */
export interface UserProfileBulkTags {
  tag_ids: Snowflake[]
}

export interface UserProfileMeta {
  /** @default false */
  is_a_match?: boolean
  /** @default false */
  is_liked?: boolean
}

export enum UserProfileSortBy {
  Age = 'age',
  FameRating = 'fame_rating',
  Distance = 'distance',
  Tags = 'tags'
}

export type QueryParamsType = Record<string | number, any>
export type ResponseFormat = keyof Omit<Body, 'body' | 'bodyUsed'>

export interface FullRequestParams extends Omit<RequestInit, 'body'> {
  /** set parameter to `true` for call `securityWorker` for this request */
  secure?: boolean
  /** request path */
  path: string
  /** content type of request body */
  type?: ContentType
  /** query params */
  query?: QueryParamsType
  /** format of response (i.e. response.json() -> format: "json") */
  format?: ResponseFormat
  /** request body */
  body?: unknown
  /** base url */
  baseUrl?: string
  /** request cancellation token */
  cancelToken?: CancelToken
}

export type RequestParams = Omit<
  FullRequestParams,
  'body' | 'method' | 'query' | 'path'
>

export interface ApiConfig<SecurityDataType = unknown> {
  baseUrl?: string
  baseApiParams?: Omit<RequestParams, 'baseUrl' | 'cancelToken' | 'signal'>
  securityWorker?: (
    securityData: SecurityDataType | null
  ) => Promise<RequestParams | void> | RequestParams | void
  customFetch?: typeof fetch
}

export interface HttpResponse<D extends unknown, E extends unknown = unknown>
  extends Response {
  data: D
  error: E
}

type CancelToken = Symbol | string | number

export enum ContentType {
  Json = 'application/json',
  FormData = 'multipart/form-data',
  UrlEncoded = 'application/x-www-form-urlencoded',
  Text = 'text/plain'
}

export class HttpClient<SecurityDataType = unknown> {
  public baseUrl: string = ''
  private securityData: SecurityDataType | null = null
  private securityWorker?: ApiConfig<SecurityDataType>['securityWorker']
  private abortControllers = new Map<CancelToken, AbortController>()
  private customFetch = (...fetchParams: Parameters<typeof fetch>) =>
    fetch(...fetchParams)

  private baseApiParams: RequestParams = {
    credentials: 'same-origin',
    headers: {},
    redirect: 'follow',
    referrerPolicy: 'no-referrer'
  }

  constructor(apiConfig: ApiConfig<SecurityDataType> = {}) {
    Object.assign(this, apiConfig)
  }

  public setSecurityData = (data: SecurityDataType | null) => {
    this.securityData = data
  }

  protected encodeQueryParam(key: string, value: any) {
    const encodedKey = encodeURIComponent(key)
    return `${encodedKey}=${encodeURIComponent(typeof value === 'number' ? value : `${value}`)}`
  }

  protected addQueryParam(query: QueryParamsType, key: string) {
    return this.encodeQueryParam(key, query[key])
  }

  protected addArrayQueryParam(query: QueryParamsType, key: string) {
    const value = query[key]
    return value.map((v: any) => this.encodeQueryParam(key, v)).join('&')
  }

  protected toQueryString(rawQuery?: QueryParamsType): string {
    const query = rawQuery || {}
    const keys = Object.keys(query).filter(
      (key) => 'undefined' !== typeof query[key]
    )
    return keys
      .map((key) =>
        Array.isArray(query[key])
          ? this.addArrayQueryParam(query, key)
          : this.addQueryParam(query, key)
      )
      .join('&')
  }

  protected addQueryParams(rawQuery?: QueryParamsType): string {
    const queryString = this.toQueryString(rawQuery)
    return queryString ? `?${queryString}` : ''
  }

  private contentFormatters: Record<ContentType, (input: any) => any> = {
    [ContentType.Json]: (input: any) =>
      input !== null && (typeof input === 'object' || typeof input === 'string')
        ? JSON.stringify(input)
        : input,
    [ContentType.Text]: (input: any) =>
      input !== null && typeof input !== 'string'
        ? JSON.stringify(input)
        : input,
    [ContentType.FormData]: (input: any) =>
      Object.keys(input || {}).reduce((formData, key) => {
        const property = input[key]
        formData.append(
          key,
          property instanceof Blob
            ? property
            : typeof property === 'object' && property !== null
              ? JSON.stringify(property)
              : `${property}`
        )
        return formData
      }, new FormData()),
    [ContentType.UrlEncoded]: (input: any) => this.toQueryString(input)
  }

  protected mergeRequestParams(
    params1: RequestParams,
    params2?: RequestParams
  ): RequestParams {
    return {
      ...this.baseApiParams,
      ...params1,
      ...(params2 || {}),
      headers: {
        ...(this.baseApiParams.headers || {}),
        ...(params1.headers || {}),
        ...((params2 && params2.headers) || {})
      }
    }
  }

  protected createAbortSignal = (
    cancelToken: CancelToken
  ): AbortSignal | undefined => {
    if (this.abortControllers.has(cancelToken)) {
      const abortController = this.abortControllers.get(cancelToken)
      if (abortController) {
        return abortController.signal
      }
      return void 0
    }

    const abortController = new AbortController()
    this.abortControllers.set(cancelToken, abortController)
    return abortController.signal
  }

  public abortRequest = (cancelToken: CancelToken) => {
    const abortController = this.abortControllers.get(cancelToken)

    if (abortController) {
      abortController.abort()
      this.abortControllers.delete(cancelToken)
    }
  }

  public request = async <T = any, E = any>({
    body,
    secure,
    path,
    type,
    query,
    format,
    baseUrl,
    cancelToken,
    ...params
  }: FullRequestParams): Promise<HttpResponse<T, E>> => {
    const secureParams =
      ((typeof secure === 'boolean' ? secure : this.baseApiParams.secure) &&
        this.securityWorker &&
        (await this.securityWorker(this.securityData))) ||
      {}
    const requestParams = this.mergeRequestParams(params, secureParams)
    const queryString = query && this.toQueryString(query)
    const payloadFormatter = this.contentFormatters[type || ContentType.Json]
    const responseFormat = format || requestParams.format

    return this.customFetch(
      `${baseUrl || this.baseUrl || ''}${path}${queryString ? `?${queryString}` : ''}`,
      {
        ...requestParams,
        headers: {
          ...(requestParams.headers || {}),
          ...(type && type !== ContentType.FormData
            ? { 'Content-Type': type }
            : {})
        },
        signal:
          (cancelToken
            ? this.createAbortSignal(cancelToken)
            : requestParams.signal) || null,
        body:
          typeof body === 'undefined' || body === null
            ? null
            : payloadFormatter(body)
      }
    ).then(async (response) => {
      const r = response.clone() as HttpResponse<T, E>
      r.data = null as unknown as T
      r.error = null as unknown as E

      const data = !responseFormat
        ? r
        : await response[responseFormat]()
            .then((data) => {
              if (r.ok) {
                r.data = data
              } else {
                r.error = data
              }
              return r
            })
            .catch((e) => {
              r.error = e
              return r
            })

      if (cancelToken) {
        this.abortControllers.delete(cancelToken)
      }

      if (!response.ok) throw data
      return data
    })
  }
}

/**
 * @title Matcha API
 * @version 0.1.0
 */
export class Api<
  SecurityDataType extends unknown
> extends HttpClient<SecurityDataType> {
  v1 = {
    /**
     * No description
     *
     * @tags health
     * @name Health
     * @request GET:/v1
     */
    health: (params: RequestParams = {}) =>
      this.request<ServerHealth, void>({
        path: `/v1`,
        method: 'GET',
        format: 'json',
        ...params
      }),

    /**
     * No description
     *
     * @tags auth
     * @name Login42
     * @summary Login with 42 account
     * @request GET:/v1/auth/oauth2/42/login
     */
    login42: (params: RequestParams = {}) =>
      this.request<OAuthResponse, void>({
        path: `/v1/auth/oauth2/42/login`,
        credentials: 'include',
        method: 'GET',
        format: 'json',
        ...params
      }),

    /**
     * No description
     *
     * @tags auth
     * @name Callback42
     * @summary Callback for 42 OAuth
     * @request GET:/v1/auth/oauth2/42/callback
     * @secure
     */
    callback42: (
      query: {
        code: string
        state: string
      },
      params: RequestParams = {}
    ) =>
      this.request<void, void>({
        path: `/v1/auth/oauth2/42/callback`,
        credentials: 'include',
        method: 'GET',
        query: query,
        secure: true,
        ...params
      }),

    /**
     * No description
     *
     * @tags auth
     * @name Register
     * @summary Register a new user
     * @request POST:/v1/auth/register
     */
    register: (data: RegisterUser, params: RequestParams = {}) =>
      this.request<void, void>({
        path: `/v1/auth/register`,
        credentials: 'include',
        method: 'POST',
        body: data,
        type: ContentType.Json,
        ...params
      }),

    /**
     * No description
     *
     * @tags auth
     * @name Login
     * @summary Login with credentials
     * @request POST:/v1/auth/login
     * @secure
     */
    login: (data: Login, params: RequestParams = {}) =>
      this.request<void, void>({
        path: `/v1/auth/login`,
        credentials: 'include',
        method: 'POST',
        body: data,
        secure: true,
        type: ContentType.Json,
        ...params
      }),

    /**
     * No description
     *
     * @tags auth
     * @name ActivateAccount
     * @summary Activate the user account
     * @request GET:/v1/auth/activate
     */
    activateAccount: (
      query: {
        token: string
      },
      params: RequestParams = {}
    ) =>
      this.request<void, void>({
        path: `/v1/auth/activate`,
        credentials: 'include',
        method: 'GET',
        query: query,
        ...params
      }),

    /**
     * No description
     *
     * @tags auth
     * @name RequestResetPassword
     * @summary Request a password reset
     * @request POST:/v1/auth/password
     */
    requestResetPassword: (
      data: RequestResetPassword,
      params: RequestParams = {}
    ) =>
      this.request<void, void>({
        path: `/v1/auth/password`,
        credentials: 'include',
        method: 'POST',
        body: data,
        type: ContentType.Json,
        ...params
      }),

    /**
     * No description
     *
     * @tags auth
     * @name ResetPassword
     * @summary Reset the user password
     * @request PUT:/v1/auth/password
     */
    resetPassword: (data: ResetPassword, params: RequestParams = {}) =>
      this.request<void, void>({
        path: `/v1/auth/password`,
        credentials: 'include',
        method: 'PUT',
        body: data,
        type: ContentType.Json,
        ...params
      }),

    /**
     * No description
     *
     * @tags auth
     * @name Logout
     * @summary Logout the current user
     * @request POST:/v1/auth/logout
     * @secure
     */
    logout: (params: RequestParams = {}) =>
      this.request<void, void>({
        path: `/v1/auth/logout`,
        credentials: 'include',
        method: 'POST',
        secure: true,
        ...params
      }),

    /**
     * No description
     *
     * @tags users
     * @name GetMe
     * @summary Get the current user
     * @request GET:/v1/users/@me
     * @secure
     */
    getMe: (params: RequestParams = {}) =>
      this.request<User, void>({
        path: `/v1/users/@me`,
        credentials: 'include',
        method: 'GET',
        secure: true,
        format: 'json',
        ...params
      }),

    /**
     * No description
     *
     * @tags users
     * @name UpdateMe
     * @summary Update the current user
     * @request PATCH:/v1/users/@me
     * @secure
     */
    updateMe: (data: UpdateUser, params: RequestParams = {}) =>
      this.request<void, void>({
        path: `/v1/users/@me`,
        credentials: 'include',
        method: 'PATCH',
        body: data,
        secure: true,
        type: ContentType.Json,
        ...params
      }),

    /**
     * No description
     *
     * @tags users
     * @name CompleteOnboarding
     * @summary Complete the onboarding process
     * @request POST:/v1/users/@me/onboarding
     * @secure
     */
    completeOnboarding: (
      data: CompleteOnboardingForm,
      params: RequestParams = {}
    ) =>
      this.request<void, void>({
        path: `/v1/users/@me/onboarding`,
        credentials: 'include',
        method: 'POST',
        body: data,
        secure: true,
        type: ContentType.FormData,
        ...params
      }),

    /**
     * No description
     *
     * @tags users
     * @name GetMyChannels
     * @summary Get the current user channels
     * @request GET:/v1/users/@me/channels
     * @secure
     */
    getMyChannels: (params: RequestParams = {}) =>
      this.request<Channel[], void>({
        path: `/v1/users/@me/channels`,
        credentials: 'include',
        method: 'GET',
        secure: true,
        format: 'json',
        ...params
      }),

    /**
     * No description
     *
     * @tags users
     * @name GetBlockedProfiles
     * @summary Get blocked user profiles
     * @request GET:/v1/users/@me/blocked
     * @secure
     */
    getBlockedProfiles: (params: RequestParams = {}) =>
      this.request<UserProfile[], void>({
        path: `/v1/users/@me/blocked`,
        credentials: 'include',
        method: 'GET',
        secure: true,
        format: 'json',
        ...params
      }),

    /**
     * No description
     *
     * @tags profiles
     * @name SearchProfile
     * @summary Search user profiles
     * @request GET:/v1/profiles/search
     * @secure
     */
    searchProfile: (
      query?: {
        /** @format double */
        latitude?: number | null
        /**
         * @format int64
         * @default 25
         */
        limit?: number
        /** @format double */
        longitude?: number | null
        /** @format int32 */
        max_age?: number | null
        /** @format int32 */
        max_fame_rating?: number | null
        /** @format int32 */
        min_age?: number | null
        /** @format int32 */
        min_fame_rating?: number | null
        /**
         * @format int64
         * @default 0
         */
        offset?: number
        /** @format double */
        radius_km?: number | null
        sort_by?: UserProfileSortBy | null
        sort_order?: SortOrder | null
        tag_ids?: Snowflake[] | null
      },
      params: RequestParams = {}
    ) =>
      this.request<PartialUserProfile[], void>({
        path: `/v1/profiles/search`,
        credentials: 'include',
        method: 'GET',
        query: query,
        secure: true,
        format: 'json',
        ...params
      }),

    /**
     * No description
     *
     * @tags profiles
     * @name RecommendProfiles
     * @summary Recommend user profiles
     * @request GET:/v1/profiles/recommend
     * @secure
     */
    recommendProfiles: (params: RequestParams = {}) =>
      this.request<PartialUserProfile[], void>({
        path: `/v1/profiles/recommend`,
        credentials: 'include',
        method: 'GET',
        secure: true,
        format: 'json',
        ...params
      }),

    /**
     * No description
     *
     * @tags profiles
     * @name GetMyProfile
     * @summary Get the current user profile
     * @request GET:/v1/profiles/@me
     * @secure
     */
    getMyProfile: (params: RequestParams = {}) =>
      this.request<UserProfile, void>({
        path: `/v1/profiles/@me`,
        credentials: 'include',
        method: 'GET',
        secure: true,
        format: 'json',
        ...params
      }),

    /**
     * No description
     *
     * @tags profiles
     * @name UpdateMyProfile
     * @summary Update the current user profile
     * @request PATCH:/v1/profiles/@me
     * @secure
     */
    updateMyProfile: (data: UpdateProfile, params: RequestParams = {}) =>
      this.request<void, void>({
        path: `/v1/profiles/@me`,
        credentials: 'include',
        method: 'PATCH',
        body: data,
        secure: true,
        type: ContentType.Json,
        ...params
      }),

    /**
     * No description
     *
     * @tags profiles
     * @name UploadProfilePicture
     * @summary Upload a picture to my profile
     * @request POST:/v1/profiles/@me/pictures
     * @secure
     */
    uploadProfilePicture: (
      data: UploadProfilePictureForm,
      params: RequestParams = {}
    ) =>
      this.request<void, void>({
        path: `/v1/profiles/@me/pictures`,
        credentials: 'include',
        method: 'POST',
        body: data,
        secure: true,
        type: ContentType.FormData,
        ...params
      }),

    /**
     * No description
     *
     * @tags profiles
     * @name DeleteProfilePicture
     * @summary Delete a picture from my profile
     * @request DELETE:/v1/profiles/@me/pictures/{picture_offset}
     * @secure
     */
    deleteProfilePicture: (pictureOffset: number, params: RequestParams = {}) =>
      this.request<void, void>({
        path: `/v1/profiles/@me/pictures/${pictureOffset}`,
        credentials: 'include',
        method: 'DELETE',
        secure: true,
        ...params
      }),

    /**
     * No description
     *
     * @tags profiles
     * @name SetDefaultProfilePicture
     * @summary Set a picture as default profile picture
     * @request PUT:/v1/profiles/@me/pictures/{picture_offset}/default
     * @secure
     */
    setDefaultProfilePicture: (
      pictureOffset: number,
      params: RequestParams = {}
    ) =>
      this.request<void, void>({
        path: `/v1/profiles/@me/pictures/${pictureOffset}/default`,
        credentials: 'include',
        method: 'PUT',
        secure: true,
        ...params
      }),

    /**
     * No description
     *
     * @tags profiles
     * @name AddTagToMyProfile
     * @summary Add a tag to my profile
     * @request PUT:/v1/profiles/@me/tags
     * @secure
     */
    addTagToMyProfile: (
      query: {
        /** A 64 bit integer unique identifier (serialized as string to avoid overflow issues) */
        tag_id: Snowflake
      },
      params: RequestParams = {}
    ) =>
      this.request<void, void>({
        path: `/v1/profiles/@me/tags`,
        credentials: 'include',
        method: 'PUT',
        query: query,
        secure: true,
        ...params
      }),

    /**
     * No description
     *
     * @tags profiles
     * @name RemoveTagFromMyProfile
     * @summary Remove a tag from my profile
     * @request DELETE:/v1/profiles/@me/tags
     * @secure
     */
    removeTagFromMyProfile: (
      query: {
        /** A 64 bit integer unique identifier (serialized as string to avoid overflow issues) */
        tag_id: Snowflake
      },
      params: RequestParams = {}
    ) =>
      this.request<void, void>({
        path: `/v1/profiles/@me/tags`,
        credentials: 'include',
        method: 'DELETE',
        query: query,
        secure: true,
        ...params
      }),

    /**
     * No description
     *
     * @tags profiles
     * @name BulkAddTagToMyProfile
     * @summary Bulk add tags to my profile
     * @request PUT:/v1/profiles/@me/tags/bulk
     * @secure
     */
    bulkAddTagToMyProfile: (
      data: UserProfileBulkTags,
      params: RequestParams = {}
    ) =>
      this.request<void, void>({
        path: `/v1/profiles/@me/tags/bulk`,
        credentials: 'include',
        method: 'PUT',
        body: data,
        secure: true,
        type: ContentType.Json,
        ...params
      }),

    /**
     * No description
     *
     * @tags profiles
     * @name BulkRemoveTagFromMyProfile
     * @summary Bulk remove tags from my profile
     * @request DELETE:/v1/profiles/@me/tags/bulk
     * @secure
     */
    bulkRemoveTagFromMyProfile: (
      data: UserProfileBulkTags,
      params: RequestParams = {}
    ) =>
      this.request<void, void>({
        path: `/v1/profiles/@me/tags/bulk`,
        credentials: 'include',
        method: 'DELETE',
        body: data,
        secure: true,
        type: ContentType.Json,
        ...params
      }),

    /**
     * No description
     *
     * @tags profiles
     * @name GetMyProfileLikes
     * @summary Get the current user profile likes
     * @request GET:/v1/profiles/@me/likes
     * @secure
     */
    getMyProfileLikes: (params: RequestParams = {}) =>
      this.request<UserProfile[], void>({
        path: `/v1/profiles/@me/likes`,
        credentials: 'include',
        method: 'GET',
        secure: true,
        format: 'json',
        ...params
      }),

    /**
     * No description
     *
     * @tags profiles
     * @name GetMyProfileMatches
     * @summary Get the current user profile matches
     * @request GET:/v1/profiles/@me/matches
     * @secure
     */
    getMyProfileMatches: (params: RequestParams = {}) =>
      this.request<UserProfile[], void>({
        path: `/v1/profiles/@me/matches`,
        credentials: 'include',
        method: 'GET',
        secure: true,
        format: 'json',
        ...params
      }),

    /**
     * No description
     *
     * @tags profiles
     * @name GetMyProfileViews
     * @summary Get the current user profile views
     * @request GET:/v1/profiles/@me/views
     * @secure
     */
    getMyProfileViews: (params: RequestParams = {}) =>
      this.request<UserProfile[], void>({
        path: `/v1/profiles/@me/views`,
        credentials: 'include',
        method: 'GET',
        secure: true,
        format: 'json',
        ...params
      }),

    /**
     * No description
     *
     * @tags profiles
     * @name GetUserProfileById
     * @summary Get the user profile by id
     * @request GET:/v1/profiles/{profile_id}
     * @secure
     */
    getUserProfileById: (profileId: string, params: RequestParams = {}) =>
      this.request<PartialUserProfile, void>({
        path: `/v1/profiles/${profileId}`,
        credentials: 'include',
        method: 'GET',
        secure: true,
        format: 'json',
        ...params
      }),

    /**
     * No description
     *
     * @tags profiles
     * @name LikeUserProfile
     * @summary Like a user profile
     * @request PUT:/v1/profiles/{profile_id}/like
     * @secure
     */
    likeUserProfile: (profileId: string, params: RequestParams = {}) =>
      this.request<void, void>({
        path: `/v1/profiles/${profileId}/like`,
        credentials: 'include',
        method: 'PUT',
        secure: true,
        ...params
      }),

    /**
     * No description
     *
     * @tags profiles
     * @name RemoveUserProfileLike
     * @summary Remove a like from a user profile
     * @request DELETE:/v1/profiles/{profile_id}/like
     * @secure
     */
    removeUserProfileLike: (profileId: string, params: RequestParams = {}) =>
      this.request<void, void>({
        path: `/v1/profiles/${profileId}/like`,
        credentials: 'include',
        method: 'DELETE',
        secure: true,
        ...params
      }),

    /**
     * No description
     *
     * @tags profiles
     * @name DislikeUserProfile
     * @summary Dislike a user profile
     * @request PUT:/v1/profiles/{profile_id}/dislike
     * @secure
     */
    dislikeUserProfile: (profileId: string, params: RequestParams = {}) =>
      this.request<void, void>({
        path: `/v1/profiles/${profileId}/dislike`,
        credentials: 'include',
        method: 'PUT',
        secure: true,
        ...params
      }),

    /**
     * No description
     *
     * @tags profiles
     * @name BlockUserProfile
     * @summary Block a user profile
     * @request PUT:/v1/profiles/{profile_id}/block
     * @secure
     */
    blockUserProfile: (profileId: string, params: RequestParams = {}) =>
      this.request<void, void>({
        path: `/v1/profiles/${profileId}/block`,
        credentials: 'include',
        method: 'PUT',
        secure: true,
        ...params
      }),

    /**
     * No description
     *
     * @tags profiles
     * @name UnblockUserProfile
     * @summary Unblock a user profile
     * @request DELETE:/v1/profiles/{profile_id}/block
     * @secure
     */
    unblockUserProfile: (profileId: string, params: RequestParams = {}) =>
      this.request<void, void>({
        path: `/v1/profiles/${profileId}/block`,
        credentials: 'include',
        method: 'DELETE',
        secure: true,
        ...params
      }),

    /**
     * No description
     *
     * @tags profiles
     * @name ReportUserProfile
     * @summary Report a user profile
     * @request POST:/v1/profiles/{profile_id}/report
     * @secure
     */
    reportUserProfile: (
      profileId: string,
      data: ReportProfile,
      params: RequestParams = {}
    ) =>
      this.request<void, void>({
        path: `/v1/profiles/${profileId}/report`,
        credentials: 'include',
        method: 'POST',
        body: data,
        secure: true,
        type: ContentType.Json,
        ...params
      }),

    /**
     * No description
     *
     * @tags cdn
     * @name GetProfileImage
     * @summary Get a profile image
     * @request GET:/v1/cdn/profile/{hash}
     * @secure
     */
    getProfileImage: (hash: string, params: RequestParams = {}) =>
      this.request<void, void>({
        path: `/v1/cdn/profile/${hash}`,
        credentials: 'include',
        method: 'GET',
        secure: true,
        ...params
      }),

    /**
     * No description
     *
     * @tags tags
     * @name GetAllTags
     * @summary Get all profile tags
     * @request GET:/v1/tags
     * @secure
     */
    getAllTags: (params: RequestParams = {}) =>
      this.request<ProfileTag[], void>({
        path: `/v1/tags`,
        credentials: 'include',
        method: 'GET',
        secure: true,
        format: 'json',
        ...params
      }),

    /**
     * No description
     *
     * @tags chat
     * @name GetChannelMessages
     * @summary Retrieve messages from a channel
     * @request GET:/v1/channels/{channel_id}/messages
     * @secure
     */
    getChannelMessages: (
      channelId: string,
      query?: {
        /**
         * @format int64
         * @default 25
         */
        limit?: number
        /**
         * @format int64
         * @default 0
         */
        offset?: number
        sort_by?: MessageSortBy | null
        sort_order?: SortOrder | null
      },
      params: RequestParams = {}
    ) =>
      this.request<Message[], void>({
        path: `/v1/channels/${channelId}/messages`,
        credentials: 'include',
        method: 'GET',
        query: query,
        secure: true,
        format: 'json',
        ...params
      }),

    /**
     * No description
     *
     * @tags chat
     * @name PostChannelMessage
     * @summary Post a message to a channel
     * @request POST:/v1/channels/{channel_id}/messages
     * @secure
     */
    postChannelMessage: (
      channelId: string,
      data: PostMessage,
      params: RequestParams = {}
    ) =>
      this.request<void, void>({
        path: `/v1/channels/${channelId}/messages`,
        credentials: 'include',
        method: 'POST',
        body: data,
        secure: true,
        type: ContentType.Json,
        ...params
      }),

    /**
     * No description
     *
     * @tags chat
     * @name GetChannelMessage
     * @summary Retrieve a specific message from a channel
     * @request GET:/v1/channels/{channel_id}/{message_id}
     * @secure
     */
    getChannelMessage: (
      channelId: string,
      messageId: string,
      params: RequestParams = {}
    ) =>
      this.request<Message, void>({
        path: `/v1/channels/${channelId}/${messageId}`,
        credentials: 'include',
        method: 'GET',
        secure: true,
        format: 'json',
        ...params
      }),

    /**
     * No description
     *
     * @tags gateway
     * @name ConnectToGateway
     * @summary Connect to the events gateway
     * @request GET:/v1/gateway
     * @secure
     */
    connectToGateway: (params: RequestParams = {}) =>
      this.request<void, void>({
        path: `/v1/gateway`,
        credentials: 'include',
        method: 'GET',
        secure: true,
        ...params
      })
  }
}
