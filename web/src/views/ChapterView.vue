<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { useWordbookStore } from '@/stores/wordbook'
import type { DisplayMode } from '@/types'

const { t } = useI18n()
const route = useRoute()
const router = useRouter()
const wordbookStore = useWordbookStore()

const wordbookId = computed(() => Number(route.params.wordbookId))
const chapterId = computed(() => Number(route.params.chapterId))

const showAddWordModal = ref(false)
const showExportMenu = ref(false)
const newWordSource = ref('')
const newWordTranslation = ref('')
const newWordNote = ref('')
const clickedWords = ref<Set<number>>(new Set())
const showTagModal = ref(false)
const editingWord = ref<{ id: number; tags: { id: number; name: string; color: string | null }[] } | null>(null)

const viewMode = ref<'study' | 'manage'>('study')
const showControlPanel = ref(false)
const selectedWords = ref<Set<number>>(new Set())
const showEditModal = ref(false)
const editingWordData = ref<{ id: number; source: string; translation: string; note: string }>({ id: 0, source: '', translation: '', note: '' })
const showBatchTagModal = ref(false)
const searchQuery = ref('')
const batchTagsToAdd = ref<Set<number>>(new Set())
const batchTagsToRemove = ref<Set<number>>(new Set())

function toggleWordClick(wordId: number) {
  if (wordbookStore.displayMode === 'bilingual') return
  if (clickedWords.value.has(wordId)) {
    clickedWords.value.delete(wordId)
  } else {
    clickedWords.value.add(wordId)
  }
  clickedWords.value = new Set(clickedWords.value)
}

function shouldShowTranslation(wordId: number): boolean {
  if (wordbookStore.displayMode === 'bilingual') return true
  return clickedWords.value.has(wordId)
}

const displayModes: { value: DisplayMode; label: string }[] = [
  { value: 'original', label: 'words.displayModes.original' },
  { value: 'translation', label: 'words.displayModes.translation' },
  { value: 'bilingual', label: 'words.displayModes.bilingual' }
]

const LINE_HEIGHT_MIN = 24
const LINE_HEIGHT_MAX = 64
const WORD_GAP_MIN = 4
const WORD_GAP_MAX = 48

onMounted(async () => {
  await wordbookStore.fetchWordbook(wordbookId.value)
  await wordbookStore.fetchChapters(wordbookId.value)
  await wordbookStore.fetchWords(chapterId.value)
  await wordbookStore.fetchTags()
  
  const chapter = wordbookStore.chapters.find((c) => c.id === chapterId.value)
  if (chapter) {
    wordbookStore.currentChapter = chapter
  }
})

async function handleAddWord() {
  if (!newWordSource.value.trim() || !newWordTranslation.value.trim()) return
  
  await wordbookStore.createWord(chapterId.value, {
    source: newWordSource.value,
    translation: newWordTranslation.value,
    note: newWordNote.value || undefined
  })
  
  newWordSource.value = ''
  newWordTranslation.value = ''
  newWordNote.value = ''
  showAddWordModal.value = false
}

function isTagSelected(tagId: number): boolean {
  return wordbookStore.selectedTags.includes(tagId)
}

function openTagModal(event: Event, wordId: number, tags: { id: number; name: string; color: string | null }[]) {
  event.stopPropagation()
  editingWord.value = { id: wordId, tags: tags || [] }
  showTagModal.value = true
}

function toggleWordSelection(wordId: number) {
  if (selectedWords.value.has(wordId)) {
    selectedWords.value.delete(wordId)
  } else {
    selectedWords.value.add(wordId)
  }
  selectedWords.value = new Set(selectedWords.value)
}

function toggleSelectAll() {
  const filtered = searchFilteredWords()
  if (selectedWords.value.size === filtered.length) {
    selectedWords.value.clear()
  } else {
    selectedWords.value = new Set(filtered.map(w => w.id))
  }
  selectedWords.value = new Set(selectedWords.value)
}

function searchFilteredWords() {
  const words = wordbookStore.filteredWords()
  if (!searchQuery.value.trim()) return words
  const query = searchQuery.value.toLowerCase()
  return words.filter(w =>
    w.source.toLowerCase().includes(query) ||
    w.translation.toLowerCase().includes(query) ||
    (w.note && w.note.toLowerCase().includes(query))
  )
}

function openEditModal(word: { id: number; source: string; translation: string; note: string | null }) {
  editingWordData.value = { id: word.id, source: word.source, translation: word.translation, note: word.note || '' }
  showEditModal.value = true
}

async function handleEditWord() {
  if (!editingWordData.value.source.trim() || !editingWordData.value.translation.trim()) return
  await wordbookStore.updateWord(chapterId.value, editingWordData.value.id, {
    source: editingWordData.value.source,
    translation: editingWordData.value.translation,
    note: editingWordData.value.note || undefined
  })
  showEditModal.value = false
}

async function handleDeleteWord(wordId: number) {
  if (!confirm(t('words.deleteConfirm'))) return
  await wordbookStore.deleteWord(chapterId.value, wordId)
}

async function handleBatchDelete() {
  if (selectedWords.value.size === 0) return
  if (!confirm(t('words.manage.deleteConfirm', { count: selectedWords.value.size }))) return
  await wordbookStore.batchDeleteWords(chapterId.value, Array.from(selectedWords.value))
  selectedWords.value.clear()
}

function toggleBatchTagAdd(tagId: number) {
  batchTagsToRemove.value.delete(tagId)
  if (batchTagsToAdd.value.has(tagId)) {
    batchTagsToAdd.value.delete(tagId)
  } else {
    batchTagsToAdd.value.add(tagId)
  }
  batchTagsToAdd.value = new Set(batchTagsToAdd.value)
  batchTagsToRemove.value = new Set(batchTagsToRemove.value)
}

function toggleBatchTagRemove(tagId: number) {
  batchTagsToAdd.value.delete(tagId)
  if (batchTagsToRemove.value.has(tagId)) {
    batchTagsToRemove.value.delete(tagId)
  } else {
    batchTagsToRemove.value.add(tagId)
  }
  batchTagsToAdd.value = new Set(batchTagsToAdd.value)
  batchTagsToRemove.value = new Set(batchTagsToRemove.value)
}

function openBatchTagModal() {
  batchTagsToAdd.value.clear()
  batchTagsToRemove.value.clear()
  showBatchTagModal.value = true
}

async function handleBatchUpdateTags() {
  if (selectedWords.value.size === 0) return
  if (batchTagsToAdd.value.size === 0 && batchTagsToRemove.value.size === 0) {
    showBatchTagModal.value = false
    return
  }
  await wordbookStore.batchUpdateTags(
    chapterId.value,
    Array.from(selectedWords.value),
    Array.from(batchTagsToAdd.value),
    Array.from(batchTagsToRemove.value)
  )
  showBatchTagModal.value = false
  batchTagsToAdd.value.clear()
  batchTagsToRemove.value.clear()
}

function isWordTagged(tagId: number): boolean {
  return editingWord.value?.tags?.some(t => t.id === tagId) ?? false
}

async function toggleWordTag(tagId: number) {
  if (!editingWord.value) return
  if (isWordTagged(tagId)) {
    await wordbookStore.removeTagFromWord(chapterId.value, editingWord.value.id, tagId)
  } else {
    await wordbookStore.addTagToWord(chapterId.value, editingWord.value.id, tagId)
  }
  const updated = wordbookStore.words.find(w => w.id === editingWord.value?.id)
  if (updated) {
    editingWord.value = { id: updated.id, tags: updated.tags || [] }
  }
}

const notebookStyle = computed(() => {
  const fontSize = Math.max(12, wordbookStore.lineHeight * 0.5)
  const tagHeight = Math.max(10, wordbookStore.lineHeight * 0.32)
  const translationFontSize = Math.max(10, wordbookStore.lineHeight * 0.35)
  const translationHeight = Math.max(14, wordbookStore.lineHeight * 0.4)
  const tagGap = Math.max(4, wordbookStore.lineHeight * 0.15)
  const translationGap = Math.max(5, wordbookStore.lineHeight * 0.18)
  return {
    '--word-gap-x': `${wordbookStore.wordGap}px`,
    '--font-size': `${fontSize}px`,
    '--main-height': `${fontSize}px`,
    '--tag-height': `${tagHeight}px`,
    '--tag-font-size': `${Math.max(8, tagHeight * 0.85)}px`,
    '--tag-gap': `${tagGap}px`,
    '--translation-font-size': `${translationFontSize}px`,
    '--translation-height': `${translationHeight}px`,
    '--translation-gap': `${translationGap}px`,
    '--row-height': `${tagHeight + tagGap + fontSize + translationGap + translationHeight}px`
  }
})

const notebookContentStyle = computed(() => {
  const style = notebookStyle.value
  const tagHeight = parseFloat(style['--tag-height'])
  const tagGap = parseFloat(style['--tag-gap'])
  const mainHeight = parseFloat(style['--main-height'])
  const rowHeight = parseFloat(style['--row-height'])
  const linePos = tagHeight + tagGap + mainHeight
  return {
    columnGap: style['--word-gap-x'],
    rowGap: '4px',
    backgroundImage: `repeating-linear-gradient(
      transparent 0,
      transparent ${linePos - 2}px,
      #8b7355 ${linePos - 2}px,
      #8b7355 ${linePos}px,
      transparent ${linePos}px,
      transparent ${rowHeight + 4}px
    )`,
    backgroundSize: `100% ${rowHeight + 4}px`
  }
})

function handleLineHeightChange(event: Event) {
  const value = Number((event.target as HTMLInputElement).value)
  wordbookStore.setLineHeight(value)
}

function handleWordGapChange(event: Event) {
  const value = Number((event.target as HTMLInputElement).value)
  wordbookStore.setWordGap(value)
}

function exportChapter(format: string) {
  window.open(`/api/export/wordbooks/${wordbookId.value}/chapters/${chapterId.value}?format=${format}`, '_blank')
  showExportMenu.value = false
}
</script>

<template>
  <div>
    <button 
      @click="router.push(`/wordbook/${wordbookId}`)" 
      class="flex items-center gap-2 text-iron-hardware-500 hover:text-iron-hardware-700 mb-6 
             transition-all duration-300 hover:-translate-x-1"
    >
      <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
      </svg>
      {{ t('chapters.backTo') }} {{ wordbookStore.currentWordbook?.name || t('nav.wordbooks') }}
    </button>
    
    <div class="flex items-center justify-between mb-4 animate-fade-in-up stagger-1">
      <h1 class="text-xl sm:text-3xl font-bold text-iron-hardware-800">
        {{ wordbookStore.currentChapter?.name || t('chapters.title') }}
      </h1>
      
      <button
        @click="showControlPanel = !showControlPanel"
        class="flex items-center gap-1 px-3 py-1.5 rounded-xl text-sm bg-washed-linen-100 text-iron-hardware-600 hover:bg-washed-linen-200 transition-all duration-300"
      >
        <svg
          class="w-4 h-4 transition-transform duration-300"
          :class="{ 'rotate-180': showControlPanel }"
          fill="none" stroke="currentColor" viewBox="0 0 24 24"
        >
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
        </svg>
        {{ t('words.controls') }}
      </button>
    </div>
    
    <div v-if="showControlPanel" class="fixed inset-0 z-30" @click="showControlPanel = false"></div>
    
    <div class="relative z-40 mb-4">
      <Transition
        enter-active-class="transition-all duration-200 ease-out"
        enter-from-class="opacity-0 -translate-y-2"
        enter-to-class="opacity-100 translate-y-0"
        leave-active-class="transition-all duration-150 ease-in"
        leave-from-class="opacity-100 translate-y-0"
        leave-to-class="opacity-0 -translate-y-2"
      >
        <div v-if="showControlPanel" class="absolute top-0 left-0 right-0 card shadow-soft-lg">
        <div class="flex flex-wrap items-center gap-2 mb-4">
          <div class="flex rounded-xl overflow-hidden border border-washed-linen-200">
            <button
              @click="viewMode = 'study'"
              :class="[
                'px-3 py-1.5 text-sm transition-all duration-300',
                viewMode === 'study'
                  ? 'bg-raw-walnut-500 text-white'
                  : 'bg-white text-iron-hardware-600 hover:bg-washed-linen-100'
              ]"
            >
              {{ t('words.manage.view') }}
            </button>
            <button
              @click="viewMode = 'manage'"
              :class="[
                'px-3 py-1.5 text-sm transition-all duration-300',
                viewMode === 'manage'
                  ? 'bg-raw-walnut-500 text-white'
                  : 'bg-white text-iron-hardware-600 hover:bg-washed-linen-100'
              ]"
            >
              {{ t('words.manage.table') }}
            </button>
          </div>
          
          <template v-if="viewMode === 'manage' && selectedWords.size > 0">
            <span class="text-sm text-iron-hardware-500">{{ t('words.manage.selected', { count: selectedWords.size }) }}</span>
            <button
              @click="handleBatchDelete"
              class="flex items-center gap-1 px-3 py-1.5 rounded-xl text-sm transition-all duration-300 bg-red-100 text-red-600 hover:bg-red-200"
            >
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
              </svg>
              {{ t('words.manage.batchDelete') }}
            </button>
            <button
              @click="openBatchTagModal"
              class="flex items-center gap-1 px-3 py-1.5 rounded-xl text-sm transition-all duration-300 bg-washed-linen-100 text-iron-hardware-600 hover:bg-washed-linen-200"
            >
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 7h.01M7 3h5c.512 0 1.024.195 1.414.586l7 7a2 2 0 010 2.828l-7 7a2 2 0 01-2.828 0l-7-7A1.994 1.994 0 013 12V7a4 4 0 014-4z" />
              </svg>
              {{ t('words.manage.batchEditTags') }}
            </button>
          </template>
          
          <div class="flex-1"></div>
          
          <div class="relative">
            <button @click="showExportMenu = !showExportMenu" class="btn-secondary text-sm py-1.5 px-3">
              {{ t('importExport.exportChapter') }}
            </button>
            <Transition
              enter-active-class="transition-all duration-200 ease-out"
              enter-from-class="opacity-0 scale-95 translate-y-1"
              enter-to-class="opacity-100 scale-100 translate-y-0"
              leave-active-class="transition-all duration-150 ease-in"
              leave-from-class="opacity-100 scale-100"
              leave-to-class="opacity-0 scale-95"
            >
              <div v-if="showExportMenu" class="absolute right-0 top-full mt-1 w-32 bg-white rounded-xl shadow-soft-lg border border-washed-linen-200 py-2 z-10">
                <button @click="exportChapter('json')" class="w-full px-3 py-2 text-left text-sm hover:bg-washed-linen-100 transition-colors">
                  {{ t('importExport.formats.json') }}
                </button>
                <button @click="exportChapter('xml')" class="w-full px-3 py-2 text-left text-sm hover:bg-washed-linen-100 transition-colors">
                  {{ t('importExport.formats.xml') }}
                </button>
                <button @click="exportChapter('tsv')" class="w-full px-3 py-2 text-left text-sm hover:bg-washed-linen-100 transition-colors">
                  {{ t('importExport.formats.tsv') }}
                </button>
              </div>
            </Transition>
          </div>
          
          <button @click="showAddWordModal = true" class="btn-primary text-sm py-1.5 px-3">
            {{ t('words.add') }}
          </button>
        </div>
        
        <template v-if="viewMode === 'study'">
          <div class="flex flex-wrap items-center gap-3 mb-4 pt-4 border-t border-washed-linen-200">
            <span class="text-sm text-iron-hardware-500">{{ t('words.display') }}:</span>
            <div class="flex rounded-xl overflow-hidden border border-washed-linen-200">
              <button
                v-for="mode in displayModes"
                :key="mode.value"
                @click="wordbookStore.setDisplayMode(mode.value)"
                :class="[
                  'px-3 py-1.5 text-sm transition-all duration-300',
                  wordbookStore.displayMode === mode.value
                    ? 'bg-raw-walnut-500 text-white'
                    : 'bg-white text-iron-hardware-600 hover:bg-washed-linen-100'
                ]"
              >
                {{ t(mode.label) }}
              </button>
            </div>
            
            <button
              @click="wordbookStore.shuffleWords()"
              class="flex items-center gap-2 px-3 py-1.5 rounded-xl text-sm transition-all duration-300 bg-washed-linen-100 text-iron-hardware-600 hover:bg-washed-linen-200"
            >
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                      d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
              </svg>
              {{ t('words.shuffle') }}
            </button>
          </div>
          
          <div class="grid grid-cols-1 sm:grid-cols-2 gap-4 mb-4">
            <div class="flex items-center gap-3">
              <span class="text-sm text-iron-hardware-500 whitespace-nowrap">{{ t('settings.lineHeight.title') }}:</span>
              <input
                type="range"
                :min="LINE_HEIGHT_MIN"
                :max="LINE_HEIGHT_MAX"
                :value="wordbookStore.lineHeight"
                @input="handleLineHeightChange"
                class="slider flex-1 min-w-0"
              />
              <span class="text-xs text-iron-hardware-400 w-8 text-right flex-shrink-0">{{ wordbookStore.lineHeight }}</span>
            </div>
            
            <div class="flex items-center gap-3">
              <span class="text-sm text-iron-hardware-500 whitespace-nowrap">{{ t('settings.wordSpacing.title') }}:</span>
              <input
                type="range"
                :min="WORD_GAP_MIN"
                :max="WORD_GAP_MAX"
                :value="wordbookStore.wordGap"
                @input="handleWordGapChange"
                class="slider flex-1 min-w-0"
              />
              <span class="text-xs text-iron-hardware-400 w-8 text-right flex-shrink-0">{{ wordbookStore.wordGap }}</span>
            </div>
          </div>
        </template>
        
        <div v-if="wordbookStore.tags.length > 0" :class="{ 'pt-4 border-t border-washed-linen-200': true }">
          <div class="flex flex-wrap items-center gap-2">
            <span class="text-sm text-iron-hardware-500">{{ t('words.filterByTag') }}</span>
            <TransitionGroup
              enter-active-class="transition-all duration-300 ease-out"
              enter-from-class="opacity-0 scale-90"
              enter-to-class="opacity-100 scale-100"
              leave-active-class="transition-all duration-200 ease-in"
              leave-from-class="opacity-100 scale-100"
              leave-to-class="opacity-0 scale-90"
            >
              <button
                v-for="tag in wordbookStore.tags"
                :key="tag.id"
                @click="wordbookStore.toggleTagFilter(tag.id)"
                :class="[
                  'tag-chip transition-all duration-300 transform hover:scale-105',
                  isTagSelected(tag.id)
                    ? 'bg-raw-walnut-500 text-white'
                    : ''
                ]"
                :style="tag.color && !isTagSelected(tag.id) ? { backgroundColor: tag.color + '20', color: tag.color } : {}"
              >
                {{ tag.name }}
              </button>
            </TransitionGroup>
            <Transition
              enter-active-class="transition-all duration-200"
              enter-from-class="opacity-0"
              enter-to-class="opacity-100"
              leave-active-class="transition-all duration-150"
              leave-from-class="opacity-100"
              leave-to-class="opacity-0"
            >
              <button
                v-if="wordbookStore.selectedTags.length > 0"
                @click="wordbookStore.selectedTags = []"
                class="text-sm text-iron-hardware-400 hover:text-iron-hardware-600 transition-colors"
              >
                {{ t('common.clear') }}
              </button>
            </Transition>
          </div>
        </div>
        
        <div v-else-if="viewMode === 'manage'" class="text-sm text-iron-hardware-400 text-center py-2 pt-4 border-t border-washed-linen-200">
          {{ t('tags.empty.title') }}
        </div>
        </div>
      </Transition>
    </div>
    
    <div v-if="wordbookStore.words.length === 0" class="card text-center py-16 animate-fade-in-up stagger-4">
      <div class="w-20 h-20 mx-auto mb-6 rounded-blob bg-washed-linen-200 flex items-center justify-center animate-float">
        <svg class="w-10 h-10 text-iron-hardware-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                d="M3 5h12M9 3v2m1.048 9.5A18.022 18.022 0 016.412 9m6.088 9h7M11 21l5-10 5 10M12.751 5C11.783 10.77 8.07 15.61 3 18.129" />
        </svg>
      </div>
      <h3 class="text-xl font-medium text-iron-hardware-700 mb-2">{{ t('words.empty.title') }}</h3>
      <p class="text-iron-hardware-500 mb-6">{{ t('words.empty.desc') }}</p>
      <button @click="showAddWordModal = true" class="btn-primary">
        {{ t('words.empty.action') }}
      </button>
    </div>
    
    <div v-else-if="wordbookStore.filteredWords().length === 0 && wordbookStore.selectedTags.length > 0"
         class="card text-center py-16 animate-fade-in-up stagger-4">
      <div class="w-20 h-20 mx-auto mb-6 rounded-blob bg-washed-linen-200 flex items-center justify-center animate-float">
        <svg class="w-10 h-10 text-iron-hardware-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
        </svg>
      </div>
      <h3 class="text-xl font-medium text-iron-hardware-700 mb-2">{{ t('words.filterEmpty.title') }}</h3>
      <p class="text-iron-hardware-500 mb-6">{{ t('words.filterEmpty.desc') }}</p>
      <button @click="wordbookStore.clearTagFilters()" class="btn-secondary">
        {{ t('common.clear') }}
      </button>
    </div>
    
    <div v-else-if="viewMode === 'study'" class="notebook-container animate-fade-in-up stagger-4" :style="notebookStyle">
      <div class="relative">
        <TransitionGroup
          tag="div"
          class="flex flex-wrap items-start max-w-full break-words"
          :style="notebookContentStyle"
          enter-active-class="transition-all duration-300 ease-out"
          enter-from-class="opacity-0"
          enter-to-class="opacity-100"
          leave-active-class="transition-all duration-200 ease-in"
          leave-from-class="opacity-100"
          leave-to-class="opacity-0"
        >
          <div
            v-for="word in wordbookStore.filteredWords()"
            :key="word.id"
            class="flex flex-col px-1"
            :style="{ height: `${notebookStyle['--row-height']}` }"
          >
            <div
              class="flex gap-0.5 items-end justify-center cursor-pointer overflow-hidden whitespace-nowrap leading-none"
              :style="{ height: notebookStyle['--tag-height'], fontSize: notebookStyle['--tag-font-size'], marginBottom: notebookStyle['--tag-gap'] }"
              @click="openTagModal($event, word.id, word.tags || [])"
            >
              <span
                v-for="tag in word.tags"
                :key="tag.id"
                class="px-1 py-0 rounded flex-shrink-0"
                :style="{ backgroundColor: (tag.color || '#b08c64') + '30', color: tag.color || '#b08c64', fontSize: '1em', lineHeight: '1.2' }"
              >{{ tag.name }}</span>
            </div>
            <div
              class="flex items-end justify-center"
              :class="{ 'cursor-pointer': wordbookStore.displayMode !== 'bilingual' }"
              :style="{ height: notebookStyle['--main-height'] }"
              @click="toggleWordClick(word.id)"
            >
              <span
                v-if="wordbookStore.displayMode !== 'translation'"
                class="font-medium text-iron-hardware-800 font-serif leading-none"
                :style="{ fontSize: notebookStyle['--font-size'] }"
              >{{ word.source }}</span>
              <span
                v-else
                class="font-medium text-iron-hardware-800 font-serif leading-none"
                :style="{ fontSize: notebookStyle['--font-size'] }"
              >{{ word.translation }}</span>
            </div>
            <div
              class="text-iron-hardware-500 overflow-hidden whitespace-nowrap flex items-start justify-center leading-none"
              :class="{ 'invisible': !shouldShowTranslation(word.id), 'cursor-pointer': wordbookStore.displayMode !== 'bilingual' }"
              :style="{ height: notebookStyle['--translation-height'], fontSize: notebookStyle['--translation-font-size'], marginTop: notebookStyle['--translation-gap'] }"
              @click="toggleWordClick(word.id)"
            >
              <template v-if="wordbookStore.displayMode === 'bilingual' || wordbookStore.displayMode === 'original'">
                {{ word.translation }}
              </template>
              <template v-else>
                {{ word.source }}
              </template>
            </div>
          </div>
        </TransitionGroup>
      </div>
    </div>
    
    <div v-else-if="viewMode === 'manage' && wordbookStore.words.length > 0" class="card animate-fade-in-up stagger-4">
      <div class="mb-4">
        <div class="relative">
          <svg class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-iron-hardware-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
          </svg>
          <input
            v-model="searchQuery"
            type="text"
            class="input-field pl-10"
            :placeholder="t('words.manage.searchPlaceholder')"
          />
        </div>
      </div>
      <div class="overflow-x-auto">
        <table class="w-full">
          <thead>
            <tr class="border-b border-washed-linen-200">
              <th class="py-3 px-4 text-left w-12">
                <input
                  type="checkbox"
                  :checked="selectedWords.size === searchFilteredWords().length && searchFilteredWords().length > 0"
                  @change="toggleSelectAll"
                  class="w-4 h-4 rounded border-iron-hardware-300 text-raw-walnut-500 focus:ring-raw-walnut-500"
                />
              </th>
              <th class="py-3 px-4 text-left text-sm font-medium text-iron-hardware-600">{{ t('words.modal.source') }}</th>
              <th class="py-3 px-4 text-left text-sm font-medium text-iron-hardware-600">{{ t('words.modal.translation') }}</th>
              <th class="py-3 px-4 text-left text-sm font-medium text-iron-hardware-600">{{ t('words.modal.note') }}</th>
              <th class="py-3 px-4 text-left text-sm font-medium text-iron-hardware-600">{{ t('tags.title') }}</th>
              <th class="py-3 px-4 text-right text-sm font-medium text-iron-hardware-600 w-32"></th>
            </tr>
          </thead>
          <tbody>
            <tr
              v-for="word in searchFilteredWords()"
              :key="word.id"
              class="border-b border-washed-linen-100 hover:bg-washed-linen-50 transition-colors"
            >
              <td class="py-3 px-4">
                <input
                  type="checkbox"
                  :checked="selectedWords.has(word.id)"
                  @change="toggleWordSelection(word.id)"
                  class="w-4 h-4 rounded border-iron-hardware-300 text-raw-walnut-500 focus:ring-raw-walnut-500"
                />
              </td>
              <td class="py-3 px-4 text-iron-hardware-800 font-medium">{{ word.source }}</td>
              <td class="py-3 px-4 text-iron-hardware-600">{{ word.translation }}</td>
              <td class="py-3 px-4 text-iron-hardware-500 text-sm">{{ word.note || '-' }}</td>
              <td class="py-3 px-4">
                <div class="flex flex-wrap gap-1">
                  <span
                    v-for="tag in word.tags"
                    :key="tag.id"
                    class="px-2 py-0.5 text-xs rounded"
                    :style="{ backgroundColor: (tag.color || '#b08c64') + '30', color: tag.color || '#b08c64' }"
                  >{{ tag.name }}</span>
                  <button
                    @click="openTagModal($event, word.id, word.tags || [])"
                    class="px-2 py-0.5 text-xs rounded bg-washed-linen-100 text-iron-hardware-500 hover:bg-washed-linen-200"
                  >+</button>
                </div>
              </td>
              <td class="py-3 px-4 text-right">
                <div class="flex justify-end gap-2">
                  <button
                    @click="openEditModal(word)"
                    class="p-1.5 rounded-lg hover:bg-washed-linen-100 text-iron-hardware-500 hover:text-iron-hardware-700 transition-colors"
                  >
                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
                    </svg>
                  </button>
                  <button
                    @click="handleDeleteWord(word.id)"
                    class="p-1.5 rounded-lg hover:bg-red-100 text-iron-hardware-500 hover:text-red-600 transition-colors"
                  >
                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                    </svg>
                  </button>
                </div>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
    
    <Teleport to="body">
      <Transition
        enter-active-class="transition-all duration-300 ease-out"
        enter-from-class="opacity-0"
        enter-to-class="opacity-100"
        leave-active-class="transition-all duration-200 ease-in"
        leave-from-class="opacity-100"
        leave-to-class="opacity-0"
      >
        <div v-if="showTagModal && editingWord" class="fixed inset-0 z-50 flex items-center justify-center">
          <div class="absolute inset-0 bg-black/20 backdrop-blur-sm" @click="showTagModal = false" />
          <div class="card relative w-full max-w-sm mx-4 animate-modal-in">
            <h2 class="text-lg font-bold text-iron-hardware-800 mb-4">{{ t('words.manageTags') }}</h2>
            <div class="flex flex-wrap gap-2">
              <button
                v-for="tag in wordbookStore.tags"
                :key="tag.id"
                @click="toggleWordTag(tag.id)"
                :class="[
                  'tag-chip transition-all duration-200',
                  isWordTagged(tag.id) ? 'ring-2 ring-raw-walnut-500' : ''
                ]"
                :style="{ backgroundColor: (tag.color || '#b08c64') + '30', color: tag.color || '#b08c64' }"
              >
                {{ tag.name }}
                <span v-if="isWordTagged(tag.id)" class="ml-1">x</span>
              </button>
            </div>
            <div v-if="wordbookStore.tags.length === 0" class="text-iron-hardware-400 text-sm py-4 text-center">
              {{ t('tags.empty.title') }}
            </div>
            <div class="mt-4 pt-4 border-t border-washed-linen-200">
              <button @click="showTagModal = false" class="btn-secondary w-full">
                {{ t('common.close') }}
              </button>
            </div>
          </div>
        </div>
      </Transition>
      
      <Transition
        enter-active-class="transition-all duration-300 ease-out"
        enter-from-class="opacity-0"
        enter-to-class="opacity-100"
        leave-active-class="transition-all duration-200 ease-in"
        leave-from-class="opacity-100"
        leave-to-class="opacity-0"
      >
        <div v-if="showAddWordModal" class="fixed inset-0 z-50 flex items-center justify-center">
          <div class="absolute inset-0 bg-black/20 backdrop-blur-sm" @click="showAddWordModal = false" />
          <div class="card relative w-full max-w-md mx-4 animate-modal-in">
            <h2 class="text-xl font-bold text-iron-hardware-800 mb-6">{{ t('words.modal.createTitle') }}</h2>
            
            <form @submit.prevent="handleAddWord" class="space-y-4">
              <div>
                <label for="source" class="block text-sm font-medium text-iron-hardware-700 mb-2">
                  {{ t('words.modal.source') }}
                </label>
                <input
                  id="source"
                  v-model="newWordSource"
                  type="text"
                  required
                  class="input-field"
                  :placeholder="t('words.modal.sourcePlaceholder')"
                />
              </div>
              
              <div>
                <label for="translation" class="block text-sm font-medium text-iron-hardware-700 mb-2">
                  {{ t('words.modal.translation') }}
                </label>
                <input
                  id="translation"
                  v-model="newWordTranslation"
                  type="text"
                  required
                  class="input-field"
                  :placeholder="t('words.modal.translationPlaceholder')"
                />
              </div>
              
              <div>
                <label for="note" class="block text-sm font-medium text-iron-hardware-700 mb-2">
                  {{ t('words.modal.note') }} <span class="text-iron-hardware-400">({{ t('common.optional') }})</span>
                </label>
                <input
                  id="note"
                  v-model="newWordNote"
                  type="text"
                  class="input-field"
                  :placeholder="t('words.modal.notePlaceholder')"
                />
              </div>
              
              <div class="flex gap-3 pt-4">
                <button type="button" @click="showAddWordModal = false" class="btn-secondary flex-1">
                  {{ t('common.cancel') }}
                </button>
                <button type="submit" class="btn-primary flex-1">
                  {{ t('words.modal.create') }}
                </button>
              </div>
            </form>
          </div>
        </div>
      </Transition>
      
      <Transition
        enter-active-class="transition-all duration-300 ease-out"
        enter-from-class="opacity-0"
        enter-to-class="opacity-100"
        leave-active-class="transition-all duration-200 ease-in"
        leave-from-class="opacity-100"
        leave-to-class="opacity-0"
      >
        <div v-if="showEditModal" class="fixed inset-0 z-50 flex items-center justify-center">
          <div class="absolute inset-0 bg-black/20 backdrop-blur-sm" @click="showEditModal = false" />
          <div class="card relative w-full max-w-md mx-4 animate-modal-in">
            <h2 class="text-xl font-bold text-iron-hardware-800 mb-6">{{ t('words.modal.editTitle') }}</h2>
            
            <form @submit.prevent="handleEditWord" class="space-y-4">
              <div>
                <label for="edit-source" class="block text-sm font-medium text-iron-hardware-700 mb-2">
                  {{ t('words.modal.source') }}
                </label>
                <input
                  id="edit-source"
                  v-model="editingWordData.source"
                  type="text"
                  required
                  class="input-field"
                  :placeholder="t('words.modal.sourcePlaceholder')"
                />
              </div>
              
              <div>
                <label for="edit-translation" class="block text-sm font-medium text-iron-hardware-700 mb-2">
                  {{ t('words.modal.translation') }}
                </label>
                <input
                  id="edit-translation"
                  v-model="editingWordData.translation"
                  type="text"
                  required
                  class="input-field"
                  :placeholder="t('words.modal.translationPlaceholder')"
                />
              </div>
              
              <div>
                <label for="edit-note" class="block text-sm font-medium text-iron-hardware-700 mb-2">
                  {{ t('words.modal.note') }} <span class="text-iron-hardware-400">({{ t('common.optional') }})</span>
                </label>
                <input
                  id="edit-note"
                  v-model="editingWordData.note"
                  type="text"
                  class="input-field"
                  :placeholder="t('words.modal.notePlaceholder')"
                />
              </div>
              
              <div class="flex gap-3 pt-4">
                <button type="button" @click="showEditModal = false" class="btn-secondary flex-1">
                  {{ t('common.cancel') }}
                </button>
                <button type="submit" class="btn-primary flex-1">
                  {{ t('words.modal.save') }}
                </button>
              </div>
            </form>
          </div>
        </div>
      </Transition>
      
      <Transition
        enter-active-class="transition-all duration-300 ease-out"
        enter-from-class="opacity-0"
        enter-to-class="opacity-100"
        leave-active-class="transition-all duration-200 ease-in"
        leave-from-class="opacity-100"
        leave-to-class="opacity-0"
      >
        <div v-if="showBatchTagModal" class="fixed inset-0 z-50 flex items-center justify-center">
          <div class="absolute inset-0 bg-black/20 backdrop-blur-sm" @click="showBatchTagModal = false" />
          <div class="card relative w-full max-w-md mx-4 animate-modal-in">
            <h2 class="text-lg font-bold text-iron-hardware-800 mb-2">{{ t('words.manage.batchEditTags') }}</h2>
            <p class="text-sm text-iron-hardware-500 mb-4">{{ t('words.manage.tagEditHint') }}</p>
            <div v-if="wordbookStore.tags.length === 0" class="text-iron-hardware-400 text-sm py-4 text-center">
              {{ t('tags.empty.title') }}
            </div>
            <div v-else class="space-y-2">
              <div
                v-for="tag in wordbookStore.tags"
                :key="tag.id"
                class="flex items-center justify-between p-2 rounded-lg border border-washed-linen-200 hover:bg-washed-linen-50 transition-colors"
              >
                <span
                  class="px-2 py-1 text-sm rounded"
                  :style="{ backgroundColor: (tag.color || '#b08c64') + '30', color: tag.color || '#b08c64' }"
                >{{ tag.name }}</span>
                <div class="flex gap-2">
                  <button
                    @click="toggleBatchTagAdd(tag.id)"
                    :class="[
                      'px-3 py-1 text-xs rounded-lg transition-all duration-200',
                      batchTagsToAdd.has(tag.id)
                        ? 'bg-green-500 text-white'
                        : 'bg-washed-linen-100 text-iron-hardware-600 hover:bg-green-100 hover:text-green-600'
                    ]"
                  >
                    + {{ t('words.manage.addTag') }}
                  </button>
                  <button
                    @click="toggleBatchTagRemove(tag.id)"
                    :class="[
                      'px-3 py-1 text-xs rounded-lg transition-all duration-200',
                      batchTagsToRemove.has(tag.id)
                        ? 'bg-red-500 text-white'
                        : 'bg-washed-linen-100 text-iron-hardware-600 hover:bg-red-100 hover:text-red-600'
                    ]"
                  >
                    - {{ t('words.manage.removeTag') }}
                  </button>
                </div>
              </div>
            </div>
            <div class="mt-4 pt-4 border-t border-washed-linen-200 flex gap-3">
              <button @click="showBatchTagModal = false" class="btn-secondary flex-1">
                {{ t('common.cancel') }}
              </button>
              <button
                @click="handleBatchUpdateTags"
                class="btn-primary flex-1"
                :disabled="batchTagsToAdd.size === 0 && batchTagsToRemove.size === 0"
              >
                {{ t('common.confirm') }}
              </button>
            </div>
          </div>
        </div>
      </Transition>
    </Teleport>
  </div>
</template>
