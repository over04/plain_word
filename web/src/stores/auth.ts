import { defineStore } from 'pinia'
import { ref } from 'vue'
import type { User } from '@/types'
import { apiClient } from '@/api/client'

export const useAuthStore = defineStore('auth', () => {
  const user = ref<User | null>(null)
  const isLoading = ref(false)

  async function checkAuth() {
    try {
      user.value = await apiClient.get<User>('/auth/me')
    } catch {
      user.value = null
    }
  }

  async function login(username: string, password: string) {
    isLoading.value = true
    try {
      user.value = await apiClient.post<User>('/auth/login', { username, password })
    } finally {
      isLoading.value = false
    }
  }

  async function register(username: string, email: string, password: string) {
    isLoading.value = true
    try {
      user.value = await apiClient.post<User>('/auth/register', { username, email, password })
    } finally {
      isLoading.value = false
    }
  }

  async function logout() {
    await apiClient.post('/auth/logout')
    user.value = null
  }

  return {
    user,
    isLoading,
    checkAuth,
    login,
    register,
    logout
  }
})