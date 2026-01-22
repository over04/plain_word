import { defineStore } from 'pinia'
import { ref } from 'vue'
import type { Wordbook, Chapter, Word, Tag, DisplayMode } from '@/types'
import { apiClient } from '@/api/client'

const LINE_HEIGHT_KEY = 'plain-word-line-height'
const WORD_GAP_KEY = 'plain-word-word-gap'

const DEFAULT_LINE_HEIGHT = 40
const DEFAULT_WORD_GAP = 16

export const useWordbookStore = defineStore('wordbook', () => {
  const wordbooks = ref<Wordbook[]>([])
  const currentWordbook = ref<Wordbook | null>(null)
  const chapters = ref<Chapter[]>([])
  const currentChapter = ref<Chapter | null>(null)
  const words = ref<Word[]>([])
  const tags = ref<Tag[]>([])
  const displayMode = ref<DisplayMode>('original')
  const selectedTags = ref<number[]>([])
  const lineHeight = ref<number>(Number(localStorage.getItem(LINE_HEIGHT_KEY)) || DEFAULT_LINE_HEIGHT)
  const wordGap = ref<number>(Number(localStorage.getItem(WORD_GAP_KEY)) || DEFAULT_WORD_GAP)

  async function fetchWordbooks() {
    wordbooks.value = await apiClient.get<Wordbook[]>('/wordbooks')
  }

  async function createWordbook(data: { name: string; description?: string }) {
    const newWordbook = await apiClient.post<Wordbook>('/wordbooks', data)
    wordbooks.value.push(newWordbook)
    return newWordbook
  }

  async function fetchWordbook(id: number | string) {
    currentWordbook.value = await apiClient.get<Wordbook>(`/wordbooks/${id}`)
  }

  async function updateWordbook(id: number | string, data: { name?: string; description?: string }) {
    currentWordbook.value = await apiClient.put<Wordbook>(`/wordbooks/${id}`, data)
    const idx = wordbooks.value.findIndex(w => w.id === Number(id))
    if (idx !== -1 && currentWordbook.value) {
      wordbooks.value[idx] = currentWordbook.value
    }
  }

  async function deleteWordbook(id: number | string) {
    await apiClient.delete(`/wordbooks/${id}`)
    wordbooks.value = wordbooks.value.filter(w => w.id !== Number(id))
  }

  async function fetchChapters(wordbookId: number | string) {
    chapters.value = await apiClient.get<Chapter[]>(`/wordbooks/${wordbookId}/chapters`)
  }

  async function createChapter(wordbookId: number | string, data: { name: string }) {
    const newChapter = await apiClient.post<Chapter>(`/wordbooks/${wordbookId}/chapters`, data)
    chapters.value.push(newChapter)
    return newChapter
  }

  async function updateChapter(wordbookId: number | string, chapterId: number | string, data: { name: string }) {
    const updated = await apiClient.put<Chapter>(`/wordbooks/${wordbookId}/chapters/${chapterId}`, data)
    const idx = chapters.value.findIndex(c => c.id === Number(chapterId))
    if (idx !== -1) {
      chapters.value[idx] = updated
    }
  }

  async function deleteChapter(wordbookId: number | string, chapterId: number | string) {
    await apiClient.delete(`/wordbooks/${wordbookId}/chapters/${chapterId}`)
    chapters.value = chapters.value.filter(c => c.id !== Number(chapterId))
  }

  async function fetchWords(chapterId: number | string) {
    words.value = await apiClient.get<Word[]>(`/chapters/${chapterId}/words`)
  }

  async function createWord(chapterId: number | string, data: { source: string; translation: string; note?: string }) {
    const newWord = await apiClient.post<Word>(`/chapters/${chapterId}/words`, data)
    words.value.push(newWord)
    return newWord
  }

  async function updateWord(chapterId: number | string, wordId: number | string, data: { source?: string; translation?: string; note?: string }) {
    const updated = await apiClient.put<Word>(`/chapters/${chapterId}/words/${wordId}`, data)
    const idx = words.value.findIndex(w => w.id === Number(wordId))
    if (idx !== -1) {
      words.value[idx] = updated
    }
    return updated
  }

  async function deleteWord(chapterId: number | string, wordId: number | string) {
    await apiClient.delete(`/chapters/${chapterId}/words/${wordId}`)
    words.value = words.value.filter(w => w.id !== Number(wordId))
  }

  async function batchDeleteWords(chapterId: number | string, wordIds: number[]) {
    await apiClient.delete(`/chapters/${chapterId}/words/batch`, { word_ids: wordIds })
    words.value = words.value.filter(w => !wordIds.includes(w.id))
  }

  async function batchUpdateTags(chapterId: number | string, wordIds: number[], addTagIds: number[], removeTagIds: number[]) {
    await apiClient.post(`/chapters/${chapterId}/words/batch/tags`, {
      word_ids: wordIds,
      add_tag_ids: addTagIds,
      remove_tag_ids: removeTagIds
    })
    await fetchWords(chapterId)
  }

  async function addTagToWord(chapterId: number | string, wordId: number | string, tagId: number) {
    const word = words.value.find(w => w.id === Number(wordId))
    if (!word) return
    const currentTagIds = word.tags?.map(t => t.id) || []
    const newTagIds = [...currentTagIds, tagId]
    const updated = await apiClient.put<Word>(`/chapters/${chapterId}/words/${wordId}/tags`, { tag_ids: newTagIds })
    const idx = words.value.findIndex(w => w.id === Number(wordId))
    if (idx !== -1) {
      words.value[idx] = updated
    }
  }

  async function removeTagFromWord(chapterId: number | string, wordId: number | string, tagId: number) {
    const word = words.value.find(w => w.id === Number(wordId))
    if (!word) return
    const newTagIds = (word.tags?.map(t => t.id) || []).filter(id => id !== tagId)
    const updated = await apiClient.put<Word>(`/chapters/${chapterId}/words/${wordId}/tags`, { tag_ids: newTagIds })
    const idx = words.value.findIndex(w => w.id === Number(wordId))
    if (idx !== -1) {
      words.value[idx] = updated
    }
  }

  async function fetchTags() {
    tags.value = await apiClient.get<Tag[]>('/tags')
  }

  async function createTag(data: { name: string; color?: string }) {
    const newTag = await apiClient.post<Tag>('/tags', data)
    tags.value.push(newTag)
    return newTag
  }

  async function updateTag(id: number | string, data: { name?: string; color?: string }) {
    const updated = await apiClient.put<Tag>(`/tags/${id}`, data)
    const idx = tags.value.findIndex(t => t.id === Number(id))
    if (idx !== -1) {
      tags.value[idx] = updated
    }
  }

  async function deleteTag(id: number | string) {
    await apiClient.delete(`/tags/${id}`)
    tags.value = tags.value.filter(t => t.id !== Number(id))
  }

  function setLineHeight(value: number) {
    lineHeight.value = value
    localStorage.setItem(LINE_HEIGHT_KEY, String(value))
  }

  function setWordGap(value: number) {
    wordGap.value = value
    localStorage.setItem(WORD_GAP_KEY, String(value))
  }

  function setDisplayMode(mode: DisplayMode) {
    displayMode.value = mode
  }

  function shuffleWords() {
    words.value = [...words.value].sort(() => Math.random() - 0.5)
  }

  function toggleTagFilter(tagId: number) {
    const idx = selectedTags.value.indexOf(tagId)
    if (idx === -1) {
      selectedTags.value.push(tagId)
    } else {
      selectedTags.value.splice(idx, 1)
    }
  }

  function clearTagFilters() {
    selectedTags.value = []
  }

  const filteredWords = () => {
    if (selectedTags.value.length === 0) return words.value
    return words.value.filter(word =>
      word.tags?.some(tag => selectedTags.value.includes(tag.id))
    )
  }

  return {
    wordbooks,
    currentWordbook,
    chapters,
    currentChapter,
    words,
    tags,
    displayMode,
    selectedTags,
    lineHeight,
    wordGap,
    fetchWordbooks,
    createWordbook,
    fetchWordbook,
    updateWordbook,
    deleteWordbook,
    fetchChapters,
    createChapter,
    updateChapter,
    deleteChapter,
    fetchWords,
    createWord,
    updateWord,
    deleteWord,
    batchDeleteWords,
    batchUpdateTags,
    addTagToWord,
    removeTagFromWord,
    fetchTags,
    createTag,
    updateTag,
    deleteTag,
    setDisplayMode,
    setLineHeight,
    setWordGap,
    shuffleWords,
    toggleTagFilter,
    clearTagFilters,
    filteredWords
  }
})