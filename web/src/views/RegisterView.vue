<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { useAuthStore } from '@/stores/auth'
import { ApiError } from '@/api/error'

const { t } = useI18n()
const router = useRouter()
const authStore = useAuthStore()

const username = ref('')
const email = ref('')
const password = ref('')
const confirmPassword = ref('')
const error = ref('')

async function handleSubmit() {
  error.value = ''
  
  if (password.value !== confirmPassword.value) {
    error.value = t('auth.register.passwordMismatch')
    return
  }
  
  try {
    await authStore.register(username.value, email.value, password.value)
    router.push('/wordbooks')
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
        <h1 class="text-2xl font-bold text-iron-hardware-800">{{ t('auth.register.title') }}</h1>
        <p class="text-iron-hardware-500 mt-2">{{ t('auth.register.subtitle') }}</p>
      </div>
      
      <form @submit.prevent="handleSubmit" class="space-y-5">
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
            {{ t('auth.register.username') }}
          </label>
          <input
            id="username"
            v-model="username"
            type="text"
            required
            class="input-field"
            :placeholder="t('auth.register.usernamePlaceholder')"
          />
        </div>
        
        <div class="space-y-2">
          <label for="email" class="block text-sm font-medium text-iron-hardware-700">
            {{ t('auth.register.email') }}
          </label>
          <input
            id="email"
            v-model="email"
            type="email"
            required
            class="input-field"
            :placeholder="t('auth.register.emailPlaceholder')"
          />
        </div>
        
        <div class="space-y-2">
          <label for="password" class="block text-sm font-medium text-iron-hardware-700">
            {{ t('auth.register.password') }}
          </label>
          <input
            id="password"
            v-model="password"
            type="password"
            required
            minlength="6"
            class="input-field"
            :placeholder="t('auth.register.passwordPlaceholder')"
          />
        </div>
        
        <div class="space-y-2">
          <label for="confirmPassword" class="block text-sm font-medium text-iron-hardware-700">
            {{ t('auth.register.confirmPassword') }}
          </label>
          <input
            id="confirmPassword"
            v-model="confirmPassword"
            type="password"
            required
            class="input-field"
            :placeholder="t('auth.register.confirmPasswordPlaceholder')"
          />
        </div>
        
        <button
          type="submit"
          :disabled="authStore.isLoading"
          class="btn-primary w-full transform hover:scale-[1.02] transition-all duration-300"
        >
          {{ authStore.isLoading ? t('auth.register.submitting') : t('auth.register.submit') }}
        </button>
      </form>
      
      <p class="text-center text-iron-hardware-500 mt-6">
        {{ t('auth.register.hasAccount') }}
        <router-link to="/login" class="text-raw-walnut-600 hover:text-raw-walnut-700 font-medium transition-colors">
          {{ t('auth.register.signIn') }}
        </router-link>
      </p>
    </div>
  </div>
</template>