<script setup lang="ts">
import { computed, ref } from 'vue'
import { useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { useAuthStore } from '@/stores/auth'
import { setLocale, availableLocales, getLocale } from '@/i18n'
import type { Locale } from '@/i18n'

const { t } = useI18n()
const router = useRouter()
const authStore = useAuthStore()

const isLoggedIn = computed(() => !!authStore.user)
const showLangMenu = ref(false)
const showMobileMenu = ref(false)
const currentLocale = ref(getLocale())

async function handleLogout() {
  await authStore.logout()
  showMobileMenu.value = false
  router.push('/login')
}

function changeLocale(locale: Locale) {
  setLocale(locale)
  currentLocale.value = locale
  showLangMenu.value = false
}

function closeMobileMenu() {
  showMobileMenu.value = false
}
</script>

<template>
  <header class="sticky top-0 z-50 backdrop-blur-md bg-washed-linen-50/80 border-b border-washed-linen-200/50 safe-area-header">
    <div class="container mx-auto px-3 sm:px-4 h-14 sm:h-16 flex items-center justify-between">
      <router-link to="/" class="flex items-center gap-2 sm:gap-3 group flex-shrink-0">
        <div class="w-8 h-8 sm:w-10 sm:h-10 rounded-blob bg-gradient-to-br from-raw-walnut-400 to-natural-oak-500
                    flex items-center justify-center text-white font-bold text-sm sm:text-lg
                    transition-all duration-500 group-hover:scale-110 group-hover:rotate-3">
          P
        </div>
        <span class="text-lg sm:text-xl font-semibold text-iron-hardware-800">{{ t('common.appName') }}</span>
      </router-link>
      
      <nav class="hidden md:flex items-center gap-4 lg:gap-6">
        <div class="relative">
          <button
            @click="showLangMenu = !showLangMenu"
            class="flex items-center gap-2 px-3 py-1.5 text-sm text-iron-hardware-600
                   hover:text-iron-hardware-800 transition-colors rounded-lg hover:bg-washed-linen-100"
          >
            {{ availableLocales.find(l => l.code === currentLocale)?.name }}
            <svg class="w-4 h-4 transition-transform" :class="{ 'rotate-180': showLangMenu }" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
            </svg>
          </button>
          <transition
            enter-active-class="transition ease-out duration-200"
            enter-from-class="opacity-0 translate-y-1"
            enter-to-class="opacity-100 translate-y-0"
            leave-active-class="transition ease-in duration-150"
            leave-from-class="opacity-100 translate-y-0"
            leave-to-class="opacity-0 translate-y-1"
          >
            <div
              v-if="showLangMenu"
              class="absolute right-0 top-full mt-2 py-2 w-32 bg-white rounded-xl shadow-soft-lg border border-washed-linen-200"
            >
              <button
                v-for="locale in availableLocales"
                :key="locale.code"
                @click="changeLocale(locale.code)"
                class="w-full px-4 py-2 text-left text-sm hover:bg-washed-linen-100 transition-colors"
                :class="{ 'text-raw-walnut-600 font-medium': currentLocale === locale.code }"
              >
                {{ locale.name }}
              </button>
            </div>
          </transition>
        </div>
        
        <template v-if="isLoggedIn">
          <router-link to="/wordbooks" class="nav-link">{{ t('nav.wordbooks') }}</router-link>
          <router-link to="/tags" class="nav-link">{{ t('nav.tags') }}</router-link>
          <router-link to="/settings" class="nav-link">{{ t('nav.settings') }}</router-link>
          <button @click="handleLogout" class="btn-secondary text-sm py-2">{{ t('nav.logout') }}</button>
        </template>
        <template v-else>
          <router-link to="/login" class="btn-secondary text-sm py-2">{{ t('nav.login') }}</router-link>
          <router-link to="/register" class="btn-primary text-sm py-2">{{ t('nav.register') }}</router-link>
        </template>
      </nav>
      
      <button
        @click="showMobileMenu = !showMobileMenu"
        class="md:hidden p-2 text-iron-hardware-600 hover:text-iron-hardware-800 transition-colors"
      >
        <svg v-if="!showMobileMenu" class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
        </svg>
        <svg v-else class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
        </svg>
      </button>
    </div>
    
  </header>
  
  <Teleport to="body">
    <transition
      enter-active-class="transition ease-out duration-200"
      enter-from-class="opacity-0"
      enter-to-class="opacity-100"
      leave-active-class="transition ease-in duration-150"
      leave-from-class="opacity-100"
      leave-to-class="opacity-0"
    >
      <div v-if="showMobileMenu" class="fixed inset-0 z-40 md:hidden" @click="closeMobileMenu"></div>
    </transition>
    
    <transition
      enter-active-class="transition ease-out duration-200"
      enter-from-class="opacity-0 -translate-y-2"
      enter-to-class="opacity-100 translate-y-0"
      leave-active-class="transition ease-in duration-150"
      leave-from-class="opacity-100 translate-y-0"
      leave-to-class="opacity-0 -translate-y-2"
    >
      <div v-if="showMobileMenu" class="fixed left-0 right-0 z-50 md:hidden bg-washed-linen-50/95 backdrop-blur-md border-b border-washed-linen-200 shadow-soft-lg safe-area-mobile-menu">
        <div class="container mx-auto px-3 py-4 space-y-3">
          <div class="flex items-center gap-2 pb-3 border-b border-washed-linen-200">
            <button
              v-for="locale in availableLocales"
              :key="locale.code"
              @click="changeLocale(locale.code)"
              class="px-3 py-1.5 text-sm rounded-lg transition-colors"
              :class="currentLocale === locale.code ? 'bg-raw-walnut-500 text-white' : 'bg-washed-linen-100 text-iron-hardware-600'"
            >
              {{ locale.name }}
            </button>
          </div>
          
          <template v-if="isLoggedIn">
            <router-link @click="closeMobileMenu" to="/wordbooks" class="block py-2 text-iron-hardware-700 hover:text-raw-walnut-600 transition-colors">
              {{ t('nav.wordbooks') }}
            </router-link>
            <router-link @click="closeMobileMenu" to="/tags" class="block py-2 text-iron-hardware-700 hover:text-raw-walnut-600 transition-colors">
              {{ t('nav.tags') }}
            </router-link>
            <router-link @click="closeMobileMenu" to="/settings" class="block py-2 text-iron-hardware-700 hover:text-raw-walnut-600 transition-colors">
              {{ t('nav.settings') }}
            </router-link>
            <button @click="handleLogout" class="w-full btn-secondary text-sm py-2 mt-2">
              {{ t('nav.logout') }}
            </button>
          </template>
          <template v-else>
            <router-link @click="closeMobileMenu" to="/login" class="block w-full btn-secondary text-sm py-2 text-center">
              {{ t('nav.login') }}
            </router-link>
            <router-link @click="closeMobileMenu" to="/register" class="block w-full btn-primary text-sm py-2 text-center">
              {{ t('nav.register') }}
            </router-link>
          </template>
        </div>
      </div>
    </transition>
  </Teleport>
</template>

<style scoped>
.nav-link {
  @apply text-iron-hardware-600 hover:text-iron-hardware-800 transition-all duration-300
         relative after:absolute after:bottom-0 after:left-0 after:w-0 after:h-0.5
         after:bg-raw-walnut-500 after:transition-all after:duration-300 hover:after:w-full;
}

.safe-area-header {
  padding-top: env(safe-area-inset-top);
}

.safe-area-mobile-menu {
  top: calc(3.5rem + env(safe-area-inset-top));
  padding-left: env(safe-area-inset-left);
  padding-right: env(safe-area-inset-right);
}

@media (min-width: 640px) {
  .safe-area-mobile-menu {
    top: calc(4rem + env(safe-area-inset-top));
  }
}
</style>