<script setup lang="ts">
import { RouterView } from 'vue-router'
import AppHeader from '@/components/AppHeader.vue'
</script>

<template>
  <div class="min-h-screen notebook-page overflow-x-hidden safe-area-container">
    <div class="blob-shape w-96 h-96 bg-raw-walnut-200 -top-48 -left-48" />
    <div class="blob-shape w-80 h-80 bg-natural-oak-200 top-1/4 -right-40 animation-delay-2000" />
    <div class="blob-shape w-64 h-64 bg-washed-linen-300 bottom-0 left-1/3 animation-delay-4000" />
    
    <div class="relative z-10 w-full max-w-full overflow-x-hidden">
      <AppHeader />
      <main class="container mx-auto px-3 sm:px-4 py-6 sm:py-8 max-w-full overflow-x-hidden safe-area-main">
        <RouterView v-slot="{ Component, route }">
          <Transition
            name="page-fade"
            mode="out-in"
          >
            <component :is="Component" :key="route.path" />
          </Transition>
        </RouterView>
      </main>
    </div>
  </div>
</template>

<style scoped>
.animation-delay-2000 {
  animation-delay: 2s;
}
.animation-delay-4000 {
  animation-delay: 4s;
}

.safe-area-container {
  padding-left: env(safe-area-inset-left);
  padding-right: env(safe-area-inset-right);
}

.safe-area-main {
  padding-bottom: env(safe-area-inset-bottom);
}
</style>