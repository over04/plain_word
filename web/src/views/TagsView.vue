<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { useWordbookStore } from '@/stores/wordbook'

const { t } = useI18n()
const wordbookStore = useWordbookStore()

const showCreateModal = ref(false)
const newTagName = ref('')
const newTagColor = ref('#b08c64')
const editingTagId = ref<number | null>(null)
const editingTagName = ref('')
const editingTagColor = ref('')

const colorOptions = [
  '#b08c64',
  '#a37a56',
  '#c4a67d',
  '#8a7251',
  '#6d6d6d',
  '#5d5d5d',
  '#b7965c',
  '#8e6b43'
]

onMounted(() => {
  wordbookStore.fetchTags()
})

async function handleCreate() {
  if (!newTagName.value.trim()) return
  
  await wordbookStore.createTag({
    name: newTagName.value,
    color: newTagColor.value
  })
  
  newTagName.value = ''
  newTagColor.value = '#b08c64'
  showCreateModal.value = false
}

function startEdit(tagId: number, name: string, color: string | null) {
  editingTagId.value = tagId
  editingTagName.value = name
  editingTagColor.value = color || '#b08c64'
}

async function saveEdit() {
  if (!editingTagId.value || !editingTagName.value.trim()) return
  
  await wordbookStore.updateTag(editingTagId.value, {
    name: editingTagName.value,
    color: editingTagColor.value
  })
  
  editingTagId.value = null
  editingTagName.value = ''
  editingTagColor.value = ''
}

function cancelEdit() {
  editingTagId.value = null
  editingTagName.value = ''
  editingTagColor.value = ''
}

async function handleDelete(id: number) {
  if (confirm(t('tags.deleteConfirm'))) {
    await wordbookStore.deleteTag(id)
  }
}
</script>

<template>
  <div>
    <div class="flex items-center justify-between mb-8 animate-fade-in-up">
      <h1 class="text-3xl font-bold text-iron-hardware-800">{{ t('tags.title') }}</h1>
      <button @click="showCreateModal = true" class="btn-primary transform hover:scale-105 transition-all duration-300">
        {{ t('tags.new') }}
      </button>
    </div>
    
    <div v-if="wordbookStore.tags.length === 0" class="card text-center py-16 animate-fade-in-up">
      <div class="w-20 h-20 mx-auto mb-6 rounded-blob bg-natural-oak-100 flex items-center justify-center animate-float">
        <svg class="w-10 h-10 text-natural-oak-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
                d="M7 7h.01M7 3h5c.512 0 1.024.195 1.414.586l7 7a2 2 0 010 2.828l-7 7a2 2 0 01-2.828 0l-7-7A1.994 1.994 0 013 12V7a4 4 0 014-4z" />
        </svg>
      </div>
      <h3 class="text-xl font-medium text-iron-hardware-700 mb-2">{{ t('tags.empty.title') }}</h3>
      <p class="text-iron-hardware-500 mb-6">{{ t('tags.empty.desc') }}</p>
      <button @click="showCreateModal = true" class="btn-primary">
        {{ t('tags.empty.action') }}
      </button>
    </div>
    
    <TransitionGroup 
      v-else 
      tag="div" 
      class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4"
      enter-active-class="transition-all duration-500 ease-out"
      enter-from-class="opacity-0 scale-95 translate-y-4"
      enter-to-class="opacity-100 scale-100 translate-y-0"
      leave-active-class="transition-all duration-300 ease-in"
      leave-from-class="opacity-100 scale-100"
      leave-to-class="opacity-0 scale-95"
    >
      <div
        v-for="tag in wordbookStore.tags"
        :key="tag.id"
        class="card group hover:scale-[1.02] transition-all duration-300"
      >
        <div v-if="editingTagId === tag.id" class="space-y-4">
          <input
            v-model="editingTagName"
            type="text"
            class="input-field"
            @keyup.enter="saveEdit"
            @keyup.escape="cancelEdit"
          />
          
          <div class="flex flex-wrap gap-2">
            <button
              v-for="color in colorOptions"
              :key="color"
              @click="editingTagColor = color"
              class="w-8 h-8 rounded-lg transition-all duration-300 hover:scale-110"
              :class="{ 'ring-2 ring-offset-2 ring-iron-hardware-400 scale-110': editingTagColor === color }"
              :style="{ backgroundColor: color }"
            />
          </div>
          
          <div class="flex gap-2">
            <button @click="cancelEdit" class="btn-secondary flex-1 text-sm py-2">
              {{ t('common.cancel') }}
            </button>
            <button @click="saveEdit" class="btn-primary flex-1 text-sm py-2">
              {{ t('common.save') }}
            </button>
          </div>
        </div>
        
        <div v-else class="flex items-center justify-between">
          <div class="flex items-center gap-3">
            <div 
              class="w-10 h-10 rounded-xl flex items-center justify-center transition-all duration-300 group-hover:scale-110"
              :style="{ backgroundColor: (tag.color || '#b08c64') + '30' }"
            >
              <div 
                class="w-4 h-4 rounded-full transition-transform duration-300 group-hover:scale-125"
                :style="{ backgroundColor: tag.color || '#b08c64' }"
              />
            </div>
            <span class="font-medium text-iron-hardware-800 group-hover:text-raw-walnut-700 transition-colors">{{ tag.name }}</span>
          </div>
          
          <div class="flex items-center gap-1 opacity-0 group-hover:opacity-100 transition-all duration-300">
            <button
              @click="startEdit(tag.id, tag.name, tag.color)"
              class="p-2 text-iron-hardware-400 hover:text-iron-hardware-600 hover:scale-110 transition-all duration-200"
            >
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
                      d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z" />
              </svg>
            </button>
            <button
              @click="handleDelete(tag.id)"
              class="p-2 text-iron-hardware-400 hover:text-red-500 hover:scale-110 transition-all duration-200"
            >
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
                      d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
              </svg>
            </button>
          </div>
        </div>
      </div>
    </TransitionGroup>
    
    <Teleport to="body">
      <Transition
        enter-active-class="transition-all duration-300 ease-out"
        enter-from-class="opacity-0"
        enter-to-class="opacity-100"
        leave-active-class="transition-all duration-200 ease-in"
        leave-from-class="opacity-100"
        leave-to-class="opacity-0"
      >
        <div v-if="showCreateModal" class="fixed inset-0 z-50 flex items-center justify-center">
          <div class="absolute inset-0 bg-black/20 backdrop-blur-sm" @click="showCreateModal = false" />
          <div class="card relative w-full max-w-md mx-4 animate-modal-in">
            <h2 class="text-xl font-bold text-iron-hardware-800 mb-6">{{ t('tags.modal.createTitle') }}</h2>
            
            <form @submit.prevent="handleCreate" class="space-y-4">
              <div>
                <label for="tagName" class="block text-sm font-medium text-iron-hardware-700 mb-2">
                  {{ t('tags.modal.name') }}
                </label>
                <input
                  id="tagName"
                  v-model="newTagName"
                  type="text"
                  required
                  class="input-field"
                  :placeholder="t('tags.modal.namePlaceholder')"
                />
              </div>
              
              <div>
                <label class="block text-sm font-medium text-iron-hardware-700 mb-2">
                  {{ t('tags.modal.color') }}
                </label>
                <div class="flex flex-wrap gap-3">
                  <button
                    v-for="color in colorOptions"
                    :key="color"
                    type="button"
                    @click="newTagColor = color"
                    class="w-10 h-10 rounded-xl transition-all duration-300 hover:scale-110"
                    :class="{ 'ring-2 ring-offset-2 ring-iron-hardware-400 scale-110': newTagColor === color }"
                    :style="{ backgroundColor: color }"
                  />
                </div>
              </div>
              
              <div class="flex gap-3 pt-4">
                <button type="button" @click="showCreateModal = false" class="btn-secondary flex-1">
                  {{ t('common.cancel') }}
                </button>
                <button type="submit" class="btn-primary flex-1">
                  {{ t('tags.modal.create') }}
                </button>
              </div>
            </form>
          </div>
        </div>
      </Transition>
    </Teleport>
  </div>
</template>