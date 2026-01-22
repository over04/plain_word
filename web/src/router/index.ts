import { createRouter, createWebHistory } from 'vue-router'
import type { RouteRecordRaw } from 'vue-router'

const routes: RouteRecordRaw[] = [
  {
    path: '/',
    name: 'home',
    component: () => import('@/views/HomeView.vue')
  },
  {
    path: '/login',
    name: 'login',
    component: () => import('@/views/LoginView.vue')
  },
  {
    path: '/register',
    name: 'register',
    component: () => import('@/views/RegisterView.vue')
  },
  {
    path: '/wordbooks',
    name: 'wordbooks',
    component: () => import('@/views/WordbooksView.vue'),
    meta: { requiresAuth: true }
  },
  {
    path: '/wordbook/:id',
    name: 'wordbook',
    component: () => import('@/views/WordbookView.vue'),
    meta: { requiresAuth: true }
  },
  {
    path: '/wordbook/:wordbookId/chapter/:chapterId',
    name: 'chapter',
    component: () => import('@/views/ChapterView.vue'),
    meta: { requiresAuth: true }
  },
  {
    path: '/tags',
    name: 'tags',
    component: () => import('@/views/TagsView.vue'),
    meta: { requiresAuth: true }
  },
  {
    path: '/settings',
    name: 'settings',
    component: () => import('@/views/SettingsView.vue'),
    meta: { requiresAuth: true }
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

router.beforeEach(async (to, _from, next) => {
  if (to.meta.requiresAuth) {
    try {
      const response = await fetch('/api/auth/me', { credentials: 'include' })
      if (response.ok) {
        next()
      } else {
        next({ name: 'login', query: { redirect: to.fullPath } })
      }
    } catch {
      next({ name: 'login', query: { redirect: to.fullPath } })
    }
  } else {
    next()
  }
})

export default router