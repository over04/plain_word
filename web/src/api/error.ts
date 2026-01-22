export type ApiErrorCode =
  | 'UNAUTHORIZED'
  | 'INVALID_CREDENTIALS'
  | 'VALIDATION_ERROR'
  | 'NOT_FOUND'
  | 'CONFLICT'
  | 'DATABASE_ERROR'
  | 'INTERNAL_ERROR'
  | 'UNKNOWN'

interface ApiErrorResponse {
  error: string
  message: string
}

export class ApiError extends Error {
  public readonly code: ApiErrorCode
  public readonly statusCode: number

  constructor(code: ApiErrorCode, message: string, statusCode: number) {
    super(message)
    this.code = code
    this.name = 'ApiError'
    this.statusCode = statusCode
  }

  static fromResponse(statusCode: number, text: string): ApiError {
    try {
      const json = JSON.parse(text) as ApiErrorResponse
      const code = (json.error || 'UNKNOWN') as ApiErrorCode
      return new ApiError(code, json.message || text, statusCode)
    } catch {
      return new ApiError('UNKNOWN', text || `HTTP error: ${statusCode}`, statusCode)
    }
  }
}