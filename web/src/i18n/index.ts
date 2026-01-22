import { createI18n } from 'vue-i18n'
import zhCN from './zh-CN'
import en from './en'

export type Locale = 'zh-CN' | 'en'

const STORAGE_KEY = 'plain-word-locale'

function getStoredLocale(): Locale {
  if (typeof localStorage !== 'undefined') {
    const stored = localStorage.getItem(STORAGE_KEY)
    if (stored === 'zh-CN' || stored === 'en') {
      return stored
    }
  }
  return 'zh-CN'
}

export const i18n = createI18n({
  legacy: false,
  locale: getStoredLocale(),
  fallbackLocale: 'zh-CN',
  messages: {
    'zh-CN': zhCN,
    'en': en
  }
})

export function setLocale(locale: Locale) {
  i18n.global.locale.value = locale
  if (typeof localStorage !== 'undefined') {
    localStorage.setItem(STORAGE_KEY, locale)
  }
  document.documentElement.lang = locale
}

export function getLocale(): Locale {
  return i18n.global.locale.value
}

export const availableLocales: { code: Locale; name: string }[] = [
  { code: 'zh-CN', name: '简体中文' },
  { code: 'en', name: 'English' }
]