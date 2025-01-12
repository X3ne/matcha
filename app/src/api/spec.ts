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

/** CompleteOnboarding */
export interface CompleteOnboarding {
  /** @format int32 */
  age: number;
  bio?: string | null;
  gender: Gender;
  location?: Location | null;
  sexual_orientation: Orientation;
  /** @default [] */
  tag_ids?: Snowflake[];
}

export enum Gender {
  Male = "male",
  Female = "female",
}

export interface Location {
  /** @format double */
  latitude: number;
  /** @format double */
  longitude: number;
}

/** OAuthResponse */
export interface OAuthResponse {
  url: string;
}

export enum Orientation {
  Male = "male",
  Female = "female",
  Bisexual = "bisexual",
}

/** ServerHealth */
export interface ServerHealth {
  status: string;
}

/**
 * Session
 * The session cookie
 * @format session
 * @example "session=123456"
 */
export type Session = string;

/**
 * A 64 bit integer unique identifier (serialized as string to avoid overflow issues)
 * @format snowflake
 * @example "1869760527605956608"
 */
export type Snowflake = string;

export interface Tag {
  /** A 64 bit integer unique identifier (serialized as string to avoid overflow issues) */
  id: Snowflake;
  name: string;
}

/** User */
export interface User {
  email: string;
  first_name: string;
  /** A 64 bit integer unique identifier (serialized as string to avoid overflow issues) */
  id: Snowflake;
  last_name: string;
  username: string;
}

/** UserProfile */
export interface UserProfile {
  /** @format int32 */
  age: number;
  avatar_hash?: string | null;
  bio?: string | null;
  gender: Gender;
  /** A 64 bit integer unique identifier (serialized as string to avoid overflow issues) */
  id: Snowflake;
  name: string;
  sexual_orientation: Orientation;
  tags: Tag[];
  /** A 64 bit integer unique identifier (serialized as string to avoid overflow issues) */
  user_id: Snowflake;
}

export type QueryParamsType = Record<string | number, any>;
export type ResponseFormat = keyof Omit<Body, "body" | "bodyUsed">;

export interface FullRequestParams extends Omit<RequestInit, "body"> {
  /** set parameter to `true` for call `securityWorker` for this request */
  secure?: boolean;
  /** request path */
  path: string;
  /** content type of request body */
  type?: ContentType;
  /** query params */
  query?: QueryParamsType;
  /** format of response (i.e. response.json() -> format: "json") */
  format?: ResponseFormat;
  /** request body */
  body?: unknown;
  /** base url */
  baseUrl?: string;
  /** request cancellation token */
  cancelToken?: CancelToken;
}

export type RequestParams = Omit<FullRequestParams, "body" | "method" | "query" | "path">;

export interface ApiConfig<SecurityDataType = unknown> {
  baseUrl?: string;
  baseApiParams?: Omit<RequestParams, "baseUrl" | "cancelToken" | "signal">;
  securityWorker?: (securityData: SecurityDataType | null) => Promise<RequestParams | void> | RequestParams | void;
  customFetch?: typeof fetch;
}

export interface HttpResponse<D extends unknown, E extends unknown = unknown> extends Response {
  data: D;
  error: E;
}

type CancelToken = Symbol | string | number;

export enum ContentType {
  Json = "application/json",
  FormData = "multipart/form-data",
  UrlEncoded = "application/x-www-form-urlencoded",
  Text = "text/plain",
}

export class HttpClient<SecurityDataType = unknown> {
  public baseUrl: string = "";
  private securityData: SecurityDataType | null = null;
  private securityWorker?: ApiConfig<SecurityDataType>["securityWorker"];
  private abortControllers = new Map<CancelToken, AbortController>();
  private customFetch = (...fetchParams: Parameters<typeof fetch>) => fetch(...fetchParams);

  private baseApiParams: RequestParams = {
    credentials: "same-origin",
    headers: {},
    redirect: "follow",
    referrerPolicy: "no-referrer",
  };

  constructor(apiConfig: ApiConfig<SecurityDataType> = {}) {
    Object.assign(this, apiConfig);
  }

  public setSecurityData = (data: SecurityDataType | null) => {
    this.securityData = data;
  };

  protected encodeQueryParam(key: string, value: any) {
    const encodedKey = encodeURIComponent(key);
    return `${encodedKey}=${encodeURIComponent(typeof value === "number" ? value : `${value}`)}`;
  }

  protected addQueryParam(query: QueryParamsType, key: string) {
    return this.encodeQueryParam(key, query[key]);
  }

  protected addArrayQueryParam(query: QueryParamsType, key: string) {
    const value = query[key];
    return value.map((v: any) => this.encodeQueryParam(key, v)).join("&");
  }

  protected toQueryString(rawQuery?: QueryParamsType): string {
    const query = rawQuery || {};
    const keys = Object.keys(query).filter((key) => "undefined" !== typeof query[key]);
    return keys
      .map((key) => (Array.isArray(query[key]) ? this.addArrayQueryParam(query, key) : this.addQueryParam(query, key)))
      .join("&");
  }

  protected addQueryParams(rawQuery?: QueryParamsType): string {
    const queryString = this.toQueryString(rawQuery);
    return queryString ? `?${queryString}` : "";
  }

  private contentFormatters: Record<ContentType, (input: any) => any> = {
    [ContentType.Json]: (input: any) =>
      input !== null && (typeof input === "object" || typeof input === "string") ? JSON.stringify(input) : input,
    [ContentType.Text]: (input: any) => (input !== null && typeof input !== "string" ? JSON.stringify(input) : input),
    [ContentType.FormData]: (input: any) =>
      Object.keys(input || {}).reduce((formData, key) => {
        const property = input[key];
        formData.append(
          key,
          property instanceof Blob
            ? property
            : typeof property === "object" && property !== null
              ? JSON.stringify(property)
              : `${property}`,
        );
        return formData;
      }, new FormData()),
    [ContentType.UrlEncoded]: (input: any) => this.toQueryString(input),
  };

  protected mergeRequestParams(params1: RequestParams, params2?: RequestParams): RequestParams {
    return {
      ...this.baseApiParams,
      ...params1,
      ...(params2 || {}),
      headers: {
        ...(this.baseApiParams.headers || {}),
        ...(params1.headers || {}),
        ...((params2 && params2.headers) || {}),
      },
    };
  }

  protected createAbortSignal = (cancelToken: CancelToken): AbortSignal | undefined => {
    if (this.abortControllers.has(cancelToken)) {
      const abortController = this.abortControllers.get(cancelToken);
      if (abortController) {
        return abortController.signal;
      }
      return void 0;
    }

    const abortController = new AbortController();
    this.abortControllers.set(cancelToken, abortController);
    return abortController.signal;
  };

  public abortRequest = (cancelToken: CancelToken) => {
    const abortController = this.abortControllers.get(cancelToken);

    if (abortController) {
      abortController.abort();
      this.abortControllers.delete(cancelToken);
    }
  };

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
      ((typeof secure === "boolean" ? secure : this.baseApiParams.secure) &&
        this.securityWorker &&
        (await this.securityWorker(this.securityData))) ||
      {};
    const requestParams = this.mergeRequestParams(params, secureParams);
    const queryString = query && this.toQueryString(query);
    const payloadFormatter = this.contentFormatters[type || ContentType.Json];
    const responseFormat = format || requestParams.format;

    return this.customFetch(`${baseUrl || this.baseUrl || ""}${path}${queryString ? `?${queryString}` : ""}`, {
      ...requestParams,
      headers: {
        ...(requestParams.headers || {}),
        ...(type && type !== ContentType.FormData ? { "Content-Type": type } : {}),
      },
      signal: (cancelToken ? this.createAbortSignal(cancelToken) : requestParams.signal) || null,
      body: typeof body === "undefined" || body === null ? null : payloadFormatter(body),
    }).then(async (response) => {
      const r = response.clone() as HttpResponse<T, E>;
      r.data = null as unknown as T;
      r.error = null as unknown as E;

      const data = !responseFormat
        ? r
        : await response[responseFormat]()
            .then((data) => {
              if (r.ok) {
                r.data = data;
              } else {
                r.error = data;
              }
              return r;
            })
            .catch((e) => {
              r.error = e;
              return r;
            });

      if (cancelToken) {
        this.abortControllers.delete(cancelToken);
      }

      if (!response.ok) throw data;
      return data;
    });
  };
}

/**
 * @title Matcha API
 * @version 0.1.0
 */
export class Api<SecurityDataType extends unknown> extends HttpClient<SecurityDataType> {
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
        method: "GET",
        format: "json",
        ...params,
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
        method: "GET",
        format: "json",
        ...params,
      }),

    /**
     * No description
     *
     * @tags auth
     * @name Callback42
     * @summary Callback for 42 OAuth
     * @request GET:/v1/auth/oauth2/42/callback
     */
    callback42: (
      query: {
        code: string;
        state: string;
      },
      params: RequestParams = {},
    ) =>
      this.request<void, void>({
        path: `/v1/auth/oauth2/42/callback`,
        method: "GET",
        query: query,
        ...params,
      }),

    /**
     * @description This endpoint can use the `session` cookie to logout the user
     *
     * @tags auth
     * @name Logout
     * @summary Logout the current user
     * @request POST:/v1/auth/logout
     */
    logout: (params: RequestParams = {}) =>
      this.request<void, void>({
        path: `/v1/auth/logout`,
        method: "POST",
        ...params,
      }),

    /**
     * No description
     *
     * @tags users
     * @name GetMe
     * @summary Get the current user
     * @request GET:/v1/users/@me
     */
    getMe: (params: RequestParams = {}) =>
      this.request<User, void>({
        path: `/v1/users/@me`,
        method: "GET",
        format: "json",
        ...params,
      }),

    /**
     * No description
     *
     * @tags users
     * @name CompleteOnboarding
     * @summary Complete the onboarding process
     * @request POST:/v1/users/@me/onboarding
     */
    completeOnboarding: (data: CompleteOnboarding, params: RequestParams = {}) =>
      this.request<void, void>({
        path: `/v1/users/@me/onboarding`,
        method: "POST",
        body: data,
        type: ContentType.Json,
        ...params,
      }),

    /**
     * No description
     *
     * @tags users
     * @name GetMyProfile
     * @summary Get the current user profile
     * @request GET:/v1/users/@me/profile
     */
    getMyProfile: (params: RequestParams = {}) =>
      this.request<UserProfile, void>({
        path: `/v1/users/@me/profile`,
        method: "GET",
        format: "json",
        ...params,
      }),
  };
}
