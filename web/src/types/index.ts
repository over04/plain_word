export interface User {
  id: number
  username: string
  email: string
  created_at: string
}

export interface Tag {
  id: number
  name: string
  color: string | null
}

export interface Wordbook {
  id: number
  name: string
  description: string | null
  cover_url: string | null
  created_at: string
  updated_at: string
}

export interface Chapter {
  id: number
  wordbook_id: number
  name: string
  sort_order: number
  created_at: string
}

export interface Word {
  id: number
  chapter_id: number
  source: string
  translation: string
  note: string | null
  sort_order: number
  created_at: string
  tags?: Tag[]
}

export type DisplayMode = 'original' | 'translation' | 'bilingual'

export interface ApiResponse<T> {
  data?: T
  error?: string
}