<script setup lang="ts">
import { ref } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { useAuthStore } from '@/stores/auth'
import { ApiError } from '@/api/error'

const { t } = useI18n()
const router = useRouter()
const route = useRoute()
const authStore = useAuthStore()

const username = ref('')
const password = ref('')
const error = ref('')

async function handleSubmit() {
  error.value = ''
  try {
    await authStore.login(username.value, password.value)
    const redirect = route.query.redirect as string || '/wordbooks'
    router.push(redirect)
  } catch (e) {
    if (e instanceof ApiError) {
      error.value = t(`errors.${e.code}`)
    } else {
      error.value = t('common.error')
    }
  }
}
</script>

<template>
  <div class="flex items-center justify-center min-h-[70vh]">
    <div class="card w-full max-w-md animate-fade-in-up">
      <div class="text-center mb-8">
        <h1 class="text-2xl font-bold text-iron-hardware-800">{{ t('auth.login.title') }}</h1>
        <p class="text-iron-hardware-500 mt-2">{{ t('auth.login.subtitle') }}</p>
      </div>
      
      <form @submit.prevent="handleSubmit" class="space-y-6">
        <transition
          enter-active-class="transition ease-out duration-300"
          enter-from-class="opacity-0 -translate-y-2"
          enter-to-class="opacity-100 translate-y-0"
          leave-active-class="transition ease-in duration-200"
          leave-from-class="opacity-100 translate-y-0"
          leave-to-class="opacity-0 -translate-y-2"
        >
          <div v-if="error" class="p-4 bg-red-50 border border-red-200 rounded-xl text-red-600 text-sm">
            {{ error }}
          </div>
        </transition>
        
        <div class="space-y-2">
          <label for="username" class="block text-sm font-medium text-iron-hardware-700">
            {{ t('auth.login.username') }}
          </label>
          <input
            id="username"
            v-model="username"
            type="text"
            required
            class="input-field"
            :placeholder="t('auth.login.usernamePlaceholder')"
          />
        </div>
        
        <div class="space-y-2">
          <label for="password" class="block text-sm font-medium text-iron-hardware-700">
            {{ t('auth.login.password') }}
          </label>
          <input
            id="password"
            v-model="password"
            type="password"
            required
            class="input-field"
            :placeholder="t('auth.login.passwordPlaceholder')"
          />
        </div>
        
        <button
          type="submit"
          :disabled="authStore.isLoading"
          class="btn-primary w-full transform hover:scale-[1.02] transition-all duration-300"
        >
          {{ authStore.isLoading ? t('auth.login.submitting') : t('auth.login.submit') }}
        </button>
      </form>
      
      <p class="text-center text-iron-hardware-500 mt-6">
        {{ t('auth.login.noAccount') }}
        <router-link to="/register" class="text-raw-walnut-600 hover:text-raw-walnut-700 font-medium transition-colors">
          {{ t('auth.login.signUp') }}
        </router-link>
      </p>
    </div>
  </div>
</template>