<script setup lang="ts">
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { useAuthStore } from '@/stores/auth'
import { useWordbookStore } from '@/stores/wordbook'
import { setLocale, availableLocales, getLocale } from '@/i18n'
import type { Locale } from '@/i18n'

const { t } = useI18n()
const authStore = useAuthStore()
const wordbookStore = useWordbookStore()
const currentLocale = ref(getLocale())

const LINE_HEIGHT_MIN = 24
const LINE_HEIGHT_MAX = 64
const WORD_GAP_MIN = 4
const WORD_GAP_MAX = 48

function changeLocale(locale: Locale) {
  setLocale(locale)
  currentLocale.value = locale
}

function handleLineHeightChange(event: Event) {
  const value = Number((event.target as HTMLInputElement).value)
  wordbookStore.setLineHeight(value)
}

function handleWordGapChange(event: Event) {
  const value = Number((event.target as HTMLInputElement).value)
  wordbookStore.setWordGap(value)
}
</script>

<template>
  <div>
    <h1 class="text-3xl font-bold text-iron-hardware-800 mb-8 animate-fade-in-up">{{ t('settings.title') }}</h1>
    
    <div class="max-w-2xl mx-auto space-y-6">
      <div class="card animate-fade-in-up animation-delay-200">
        <h2 class="text-lg font-semibold text-iron-hardware-800 mb-4">{{ t('settings.account.title') }}</h2>
        
        <div class="space-y-4">
          <div class="flex items-center justify-between py-2 border-b border-washed-linen-100">
            <label class="text-sm font-medium text-iron-hardware-500">{{ t('settings.account.username') }}</label>
            <div class="text-iron-hardware-800 font-medium">{{ authStore.user?.username }}</div>
          </div>
          
          <div class="flex items-center justify-between py-2 border-b border-washed-linen-100">
            <label class="text-sm font-medium text-iron-hardware-500">{{ t('settings.account.email') }}</label>
            <div class="text-iron-hardware-800">{{ authStore.user?.email }}</div>
          </div>
          
          <div class="flex items-center justify-between py-2">
            <label class="text-sm font-medium text-iron-hardware-500">{{ t('settings.account.memberSince') }}</label>
            <div class="text-iron-hardware-800">
              {{ authStore.user?.created_at ? new Date(authStore.user.created_at).toLocaleDateString() : '-' }}
            </div>
          </div>
        </div>
      </div>
      
      <div class="card animate-fade-in-up animation-delay-400">
        <h2 class="text-lg font-semibold text-iron-hardware-800 mb-4">{{ t('settings.display.title') }}</h2>
        
        <div class="space-y-6">
          <div>
            <div class="flex items-center justify-between mb-2">
              <label class="text-sm font-medium text-iron-hardware-600">{{ t('settings.lineHeight.title') }}</label>
              <span class="text-sm text-iron-hardware-400">{{ wordbookStore.lineHeight }}px</span>
            </div>
            <input
              type="range"
              :min="LINE_HEIGHT_MIN"
              :max="LINE_HEIGHT_MAX"
              :value="wordbookStore.lineHeight"
              @input="handleLineHeightChange"
              class="slider w-full"
            />
          </div>
          
          <div>
            <div class="flex items-center justify-between mb-2">
              <label class="text-sm font-medium text-iron-hardware-600">{{ t('settings.wordSpacing.title') }}</label>
              <span class="text-sm text-iron-hardware-400">{{ wordbookStore.wordGap }}px</span>
            </div>
            <input
              type="range"
              :min="WORD_GAP_MIN"
              :max="WORD_GAP_MAX"
              :value="wordbookStore.wordGap"
              @input="handleWordGapChange"
              class="slider w-full"
            />
          </div>
        </div>
      </div>
      
      <div class="card animate-fade-in-up animation-delay-500">
        <h2 class="text-lg font-semibold text-iron-hardware-800 mb-4">{{ t('settings.language.title') }}</h2>
        
        <div class="flex flex-wrap gap-3">
          <button
            v-for="locale in availableLocales"
            :key="locale.code"
            @click="changeLocale(locale.code)"
            :class="[
              'px-6 py-3 rounded-xl font-medium transition-all duration-300 transform hover:scale-105',
              currentLocale === locale.code
                ? 'bg-raw-walnut-500 text-white shadow-soft'
                : 'bg-washed-linen-100 text-iron-hardware-600 hover:bg-washed-linen-200'
            ]"
          >
            {{ locale.name }}
          </button>
        </div>
      </div>
      
      <div class="card animate-fade-in-up animation-delay-700">
        <h2 class="text-lg font-semibold text-iron-hardware-800 mb-4">{{ t('settings.about.title') }}</h2>
        
        <p class="text-iron-hardware-600 mb-4 leading-relaxed">
          {{ t('settings.about.desc') }}
        </p>
        
        <div class="text-sm text-iron-hardware-400 flex items-center gap-2">
          <span class="inline-flex items-center px-2 py-1 rounded-lg bg-washed-linen-100">
            {{ t('settings.about.version') }} 1.0.0
          </span>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.animation-delay-200 {
  animation-delay: 0.2s;
}
.animation-delay-400 {
  animation-delay: 0.4s;
}
.animation-delay-500 {
  animation-delay: 0.5s;
}
.animation-delay-700 {
  animation-delay: 0.7s;
}

.slider {
  -webkit-appearance: none;
  appearance: none;
  height: 6px;
  background: linear-gradient(to right, #c9b99a, #8b7355);
  border-radius: 3px;
  outline: none;
  cursor: pointer;
}

.slider::-webkit-slider-thumb {
  -webkit-appearance: none;
  appearance: none;
  width: 18px;
  height: 18px;
  border-radius: 50%;
  background: #8b7355;
  cursor: pointer;
  border: 2px solid white;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.2);
  transition: transform 0.2s ease;
}

.slider::-webkit-slider-thumb:hover {
  transform: scale(1.15);
}

.slider::-moz-range-thumb {
  width: 16px;
  height: 16px;
  border-radius: 50%;
  background: #8b7355;
  cursor: pointer;
  border: 2px solid white;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.2);
}
</style>