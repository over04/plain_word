<script setup lang="ts">
import { ref, computed } from 'vue'
import { useI18n } from 'vue-i18n'
import type { Word, Tag, DisplayMode } from '@/types'

const { t } = useI18n()

const props = defineProps<{
  word: Word
  displayMode: DisplayMode
  lineHeight?: number
  tags: Tag[]
}>()

const emit = defineEmits<{
  delete: [wordId: number]
  addTag: [wordId: number, tagId: number]
  removeTag: [wordId: number, tagId: number]
}>()

const isRevealed = ref(false)
const showTagMenu = ref(false)

const showSource = computed(() =>
  props.displayMode === 'original' || props.displayMode === 'bilingual' || isRevealed.value
)

const showTranslation = computed(() =>
  props.displayMode === 'translation' || props.displayMode === 'bilingual' || isRevealed.value
)

const availableTags = computed(() =>
  props.tags.filter((tag: Tag) => !props.word.tags?.some((wt: Tag) => wt.id === tag.id))
)

const DEFAULT_LINE_HEIGHT = 40

const fontSizeClass = computed(() => {
  const lh = props.lineHeight || DEFAULT_LINE_HEIGHT
  if (lh <= 32) return 'text-sm'
  if (lh >= 48) return 'text-xl'
  return 'text-lg'
})

const chineseFontSizeClass = computed(() => {
  const lh = props.lineHeight || DEFAULT_LINE_HEIGHT
  if (lh <= 32) return 'text-xs'
  if (lh >= 48) return 'text-lg'
  return 'text-base'
})

function handleReveal() {
  if (props.displayMode !== 'bilingual') {
    isRevealed.value = !isRevealed.value
  }
}

function handleAddTag(tagId: number) {
  emit('addTag', props.word.id, tagId)
  showTagMenu.value = false
}

function handleRemoveTag(tagId: number) {
  emit('removeTag', props.word.id, tagId)
}
</script>

<template>
  <div class="word-card group" @click="handleReveal">
    <div class="flex items-start justify-between gap-2 mb-3">
      <div class="flex-1">
        <template v-if="displayMode === 'original' || displayMode === 'bilingual'">
          <div class="word-source" :class="fontSizeClass">{{ word.source }}</div>
          <div v-if="word.note" class="word-note text-xs text-iron-hardware-400">{{ word.note }}</div>
        </template>
        <template v-else>
          <Transition
            enter-active-class="transition-all duration-300 ease-out"
            enter-from-class="opacity-0 translate-y-2"
            enter-to-class="opacity-100 translate-y-0"
            leave-active-class="transition-all duration-200 ease-in"
            leave-from-class="opacity-100"
            leave-to-class="opacity-0"
          >
            <div v-if="showSource" class="word-source" :class="fontSizeClass">{{ word.source }}</div>
            <div v-else class="word-line cursor-pointer hover:border-raw-walnut-400" :title="t('words.clickToReveal')" />
          </Transition>
        </template>
      </div>
      
      <div class="flex items-center gap-1 opacity-0 group-hover:opacity-100 transition-all duration-300">
        <div class="relative">
          <button
            @click.stop="showTagMenu = !showTagMenu"
            class="p-1.5 text-iron-hardware-400 hover:text-natural-oak-600 hover:scale-110 transition-all duration-200"
          >
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
                    d="M7 7h.01M7 3h5c.512 0 1.024.195 1.414.586l7 7a2 2 0 010 2.828l-7 7a2 2 0 01-2.828 0l-7-7A1.994 1.994 0 013 12V7a4 4 0 014-4z" />
            </svg>
          </button>
          
          <Transition
            enter-active-class="transition-all duration-200 ease-out"
            enter-from-class="opacity-0 scale-95 translate-y-1"
            enter-to-class="opacity-100 scale-100 translate-y-0"
            leave-active-class="transition-all duration-150 ease-in"
            leave-from-class="opacity-100 scale-100"
            leave-to-class="opacity-0 scale-95"
          >
            <div 
              v-if="showTagMenu" 
              class="absolute right-0 top-full mt-1 w-48 bg-white rounded-xl shadow-soft-lg border border-washed-linen-200 py-2 z-10"
              @click.stop
            >
              <div v-if="availableTags.length === 0" class="px-3 py-2 text-sm text-iron-hardware-400">
                {{ t('words.tags.noMore') }}
              </div>
              <button
                v-for="tag in availableTags"
                :key="tag.id"
                @click="handleAddTag(tag.id)"
                class="w-full px-3 py-2 text-left text-sm hover:bg-washed-linen-100 flex items-center gap-2 transition-colors"
              >
                <span 
                  class="w-3 h-3 rounded-full transition-transform hover:scale-125" 
                  :style="{ backgroundColor: tag.color || '#b08c64' }"
                />
                {{ tag.name }}
              </button>
            </div>
          </Transition>
        </div>
        
        <button
          @click.stop="emit('delete', word.id)"
          class="p-1.5 text-iron-hardware-400 hover:text-red-500 hover:scale-110 transition-all duration-200"
        >
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
                  d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
          </svg>
        </button>
      </div>
    </div>
    
    <template v-if="displayMode === 'translation'">
      <div class="word-translation border-t-0 pt-0 mt-0" :class="chineseFontSizeClass">{{ word.translation }}</div>
    </template>
    <template v-else-if="displayMode === 'bilingual'">
      <div class="word-translation" :class="chineseFontSizeClass">{{ word.translation }}</div>
    </template>
    <template v-else>
      <Transition
        enter-active-class="transition-all duration-300 ease-out"
        enter-from-class="opacity-0 translate-y-2"
        enter-to-class="opacity-100 translate-y-0"
        leave-active-class="transition-all duration-200 ease-in"
        leave-from-class="opacity-100"
        leave-to-class="opacity-0"
      >
        <div v-if="showTranslation" class="word-translation" :class="chineseFontSizeClass">{{ word.translation }}</div>
        <div v-else class="word-line cursor-pointer hover:border-raw-walnut-400" :title="t('words.clickToReveal')" />
      </Transition>
    </template>
    
    <TransitionGroup
      v-if="word.tags && word.tags.length > 0"
      tag="div"
      class="flex flex-wrap gap-1.5 mt-3 pt-3 border-t border-washed-linen-200/50"
      enter-active-class="transition-all duration-300 ease-out"
      enter-from-class="opacity-0 scale-90"
      enter-to-class="opacity-100 scale-100"
      leave-active-class="transition-all duration-200 ease-in"
      leave-from-class="opacity-100 scale-100"
      leave-to-class="opacity-0 scale-90"
    >
      <span
        v-for="tag in word.tags"
        :key="tag.id"
        class="inline-flex items-center gap-1 px-2 py-0.5 rounded-full text-xs cursor-pointer 
               hover:opacity-80 transition-all duration-200 hover:scale-105"
        :style="{ 
          backgroundColor: (tag.color || '#b08c64') + '20', 
          color: tag.color || '#b08c64' 
        }"
        @click.stop="handleRemoveTag(tag.id)"
        :title="t('words.tags.remove')"
      >
        {{ tag.name }}
        <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
        </svg>
      </span>
    </TransitionGroup>
  </div>
</template>