<script setup lang="ts">
import { onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { useAuthStore } from '@/stores/auth'

const { t } = useI18n()
const router = useRouter()
const authStore = useAuthStore()

onMounted(async () => {
  await authStore.checkAuth()
  if (authStore.user) {
    router.push('/wordbooks')
  }
})
</script>

<template>
  <div class="flex flex-col items-center justify-center min-h-[70vh] text-center">
    <div class="relative animate-fade-in-up">
      <div class="absolute -inset-4 bg-gradient-to-r from-raw-walnut-200 via-natural-oak-200 to-washed-linen-200 
                  rounded-blob blur-2xl opacity-60 animate-blob" />
      <h1 class="relative text-5xl md:text-7xl font-bold text-iron-hardware-800 mb-6">
        {{ t('home.title') }}
      </h1>
    </div>
    
    <p class="text-xl text-iron-hardware-600 mb-8 max-w-lg text-balance animate-fade-in-up animation-delay-200">
      {{ t('home.subtitle') }}
    </p>
    
    <div class="flex gap-4 animate-fade-in-up animation-delay-400">
      <router-link to="/register" class="btn-primary transform hover:scale-105 transition-all duration-300">
        {{ t('home.getStarted') }}
      </router-link>
      <router-link to="/login" class="btn-secondary transform hover:scale-105 transition-all duration-300">
        {{ t('home.signIn') }}
      </router-link>
    </div>
    
    <div class="mt-16 grid grid-cols-1 md:grid-cols-3 gap-8 max-w-4xl">
      <div class="card text-left animate-fade-in-up animation-delay-600 hover:scale-105 transition-all duration-500">
        <div class="w-12 h-12 rounded-organic bg-raw-walnut-100 flex items-center justify-center mb-4
                    group-hover:scale-110 transition-transform duration-300">
          <svg class="w-6 h-6 text-raw-walnut-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
                  d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253" />
          </svg>
        </div>
        <h3 class="text-lg font-semibold text-iron-hardware-800 mb-2">{{ t('home.features.wordbooks.title') }}</h3>
        <p class="text-iron-hardware-500">{{ t('home.features.wordbooks.desc') }}</p>
      </div>
      
      <div class="card text-left animate-fade-in-up animation-delay-800 hover:scale-105 transition-all duration-500">
        <div class="w-12 h-12 rounded-organic bg-natural-oak-100 flex items-center justify-center mb-4">
          <svg class="w-6 h-6 text-natural-oak-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
                  d="M7 7h.01M7 3h5c.512 0 1.024.195 1.414.586l7 7a2 2 0 010 2.828l-7 7a2 2 0 01-2.828 0l-7-7A1.994 1.994 0 013 12V7a4 4 0 014-4z" />
          </svg>
        </div>
        <h3 class="text-lg font-semibold text-iron-hardware-800 mb-2">{{ t('home.features.tags.title') }}</h3>
        <p class="text-iron-hardware-500">{{ t('home.features.tags.desc') }}</p>
      </div>
      
      <div class="card text-left animate-fade-in-up animation-delay-1000 hover:scale-105 transition-all duration-500">
        <div class="w-12 h-12 rounded-organic bg-washed-linen-200 flex items-center justify-center mb-4">
          <svg class="w-6 h-6 text-iron-hardware-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
                  d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
                  d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" />
          </svg>
        </div>
        <h3 class="text-lg font-semibold text-iron-hardware-800 mb-2">{{ t('home.features.modes.title') }}</h3>
        <p class="text-iron-hardware-500">{{ t('home.features.modes.desc') }}</p>
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
.animation-delay-600 {
  animation-delay: 0.6s;
}
.animation-delay-800 {
  animation-delay: 0.8s;
}
.animation-delay-1000 {
  animation-delay: 1s;
}
</style>