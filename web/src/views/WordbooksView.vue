<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { useWordbookStore } from '@/stores/wordbook'

const { t } = useI18n()
const wordbookStore = useWordbookStore()

const showCreateModal = ref(false)
const showEditModal = ref(false)
const showImportModal = ref(false)
const exportingWordbookId = ref<number | null>(null)
const newWordbookName = ref('')
const newWordbookDescription = ref('')
const editingId = ref<number | null>(null)
const editName = ref('')
const editDescription = ref('')
const importFile = ref<File | null>(null)
const importName = ref('')
const isImporting = ref(false)
const importResult = ref<{ chapters_created: number; words_created: number } | null>(null)

onMounted(() => {
  wordbookStore.fetchWordbooks()
})

async function handleCreate() {
  if (!newWordbookName.value.trim()) return
  
  await wordbookStore.createWordbook({
    name: newWordbookName.value,
    description: newWordbookDescription.value || undefined
  })
  
  newWordbookName.value = ''
  newWordbookDescription.value = ''
  showCreateModal.value = false
}

async function handleDelete(id: number) {
  if (confirm(t('wordbooks.deleteConfirm'))) {
    await wordbookStore.deleteWordbook(id)
  }
}

function startEdit(event: Event, wordbook: { id: number; name: string; description?: string | null }) {
  event.preventDefault()
  editingId.value = wordbook.id
  editName.value = wordbook.name
  editDescription.value = wordbook.description || ''
  showEditModal.value = true
}

async function handleEdit() {
  if (!editingId.value || !editName.value.trim()) return
  
  await wordbookStore.updateWordbook(editingId.value, {
    name: editName.value,
    description: editDescription.value || undefined
  })
  
  showEditModal.value = false
  editingId.value = null
  editName.value = ''
  editDescription.value = ''
}

function handleFileChange(event: Event) {
  const target = event.target as HTMLInputElement
  if (target.files && target.files.length > 0) {
    importFile.value = target.files[0]
  }
}

async function handleImportWordbook() {
  if (!importFile.value || !importName.value.trim()) return
  
  isImporting.value = true
  try {
    const formData = new FormData()
    formData.append('file', importFile.value)
    formData.append('name', importName.value)
    
    const response = await fetch('/api/import/wordbooks', {
      method: 'POST',
      body: formData,
      credentials: 'include'
    })
    
    if (!response.ok) {
      throw new Error('Import failed')
    }
    
    importResult.value = await response.json()
    await wordbookStore.fetchWordbooks()
  } catch (error) {
    console.error('Import error:', error)
  } finally {
    isImporting.value = false
  }
}

function closeImportModal() {
  showImportModal.value = false
  importFile.value = null
  importName.value = ''
  importResult.value = null
}

function downloadTemplate(format: string) {
  window.open(`/api/import/templates?format=${format}&target=wordbook`, '_blank')
}

function toggleExportMenu(event: Event, wordbookId: number) {
  event.preventDefault()
  exportingWordbookId.value = exportingWordbookId.value === wordbookId ? null : wordbookId
}

function exportWordbook(event: Event, wordbookId: number, format: string) {
  event.preventDefault()
  window.open(`/api/export/wordbooks/${wordbookId}?format=${format}`, '_blank')
  exportingWordbookId.value = null
}
</script>

<template>
  <div>
    <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4 mb-8 animate-fade-in-up">
      <h1 class="text-2xl sm:text-3xl font-bold text-iron-hardware-800">{{ t('wordbooks.title') }}</h1>
      <div class="flex items-center gap-2 flex-shrink-0">
        <button @click="showImportModal = true" class="btn-secondary text-xs sm:text-sm px-3 sm:px-6 py-2 sm:py-3 transform hover:scale-105 transition-all duration-300">
          {{ t('importExport.importWordbook') }}
        </button>
        <button @click="showCreateModal = true" class="btn-primary text-xs sm:text-sm px-3 sm:px-6 py-2 sm:py-3 transform hover:scale-105 transition-all duration-300">
          {{ t('wordbooks.new') }}
        </button>
      </div>
    </div>
    
    <div v-if="wordbookStore.wordbooks.length === 0" class="text-center py-16 animate-fade-in-up">
      <div class="w-24 h-24 mx-auto mb-6 rounded-blob bg-washed-linen-200 flex items-center justify-center animate-float">
        <svg class="w-12 h-12 text-iron-hardware-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
                d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253" />
        </svg>
      </div>
      <h3 class="text-xl font-medium text-iron-hardware-700 mb-2">{{ t('wordbooks.empty.title') }}</h3>
      <p class="text-iron-hardware-500 mb-6">{{ t('wordbooks.empty.desc') }}</p>
      <button @click="showCreateModal = true" class="btn-primary">
        {{ t('wordbooks.empty.action') }}
      </button>
    </div>
    
    <TransitionGroup 
      v-else 
      tag="div" 
      class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6"
      enter-active-class="transition-all duration-500 ease-out"
      enter-from-class="opacity-0 translate-y-4 scale-95"
      enter-to-class="opacity-100 translate-y-0 scale-100"
      leave-active-class="transition-all duration-300 ease-in"
      leave-from-class="opacity-100 scale-100"
      leave-to-class="opacity-0 scale-95"
    >
      <router-link
        v-for="wordbook in wordbookStore.wordbooks"
        :key="wordbook.id"
        :to="`/wordbook/${wordbook.id}`"
        class="card group hover:scale-[1.02] transition-all duration-500"
      >
        <div class="flex items-start justify-between mb-4">
          <div class="w-14 h-14 rounded-organic bg-gradient-to-br from-raw-walnut-200 to-natural-oak-200 
                      flex items-center justify-center text-2xl font-bold text-raw-walnut-700
                      group-hover:scale-110 group-hover:rotate-3 transition-all duration-500">
            {{ wordbook.name.charAt(0).toUpperCase() }}
          </div>
          <div class="flex gap-1">
            <div class="relative">
              <button
                @click="toggleExportMenu($event, wordbook.id)"
                class="opacity-0 group-hover:opacity-100 p-2 text-iron-hardware-400 hover:text-raw-walnut-600
                       transition-all duration-300 transform hover:scale-110"
                :title="t('importExport.exportWordbook')"
              >
                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                        d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" />
                </svg>
              </button>
              <Transition
                enter-active-class="transition-all duration-200 ease-out"
                enter-from-class="opacity-0 scale-95 translate-y-1"
                enter-to-class="opacity-100 scale-100 translate-y-0"
                leave-active-class="transition-all duration-150 ease-in"
                leave-from-class="opacity-100 scale-100"
                leave-to-class="opacity-0 scale-95"
              >
                <div v-if="exportingWordbookId === wordbook.id" class="absolute right-0 top-full mt-1 w-32 bg-white rounded-xl shadow-soft-lg border border-washed-linen-200 py-2 z-20">
                  <button @click="exportWordbook($event, wordbook.id, 'json')" class="w-full px-3 py-2 text-left text-sm hover:bg-washed-linen-100 transition-colors">
                    {{ t('importExport.formats.json') }}
                  </button>
                  <button @click="exportWordbook($event, wordbook.id, 'xml')" class="w-full px-3 py-2 text-left text-sm hover:bg-washed-linen-100 transition-colors">
                    {{ t('importExport.formats.xml') }}
                  </button>
                  <button @click="exportWordbook($event, wordbook.id, 'xlsx')" class="w-full px-3 py-2 text-left text-sm hover:bg-washed-linen-100 transition-colors">
                    {{ t('importExport.formats.xlsx') }}
                  </button>
                  <button @click="exportWordbook($event, wordbook.id, 'csv')" class="w-full px-3 py-2 text-left text-sm hover:bg-washed-linen-100 transition-colors">
                    {{ t('importExport.formats.csv') }}
                  </button>
                </div>
              </Transition>
            </div>
            <button
              @click="startEdit($event, wordbook)"
              class="opacity-0 group-hover:opacity-100 p-2 text-iron-hardware-400 hover:text-raw-walnut-600
                     transition-all duration-300 transform hover:scale-110"
            >
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                      d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z" />
              </svg>
            </button>
            <button
              @click.prevent="handleDelete(wordbook.id)"
              class="opacity-0 group-hover:opacity-100 p-2 text-iron-hardware-400 hover:text-red-500
                     transition-all duration-300 transform hover:scale-110"
            >
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                      d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
              </svg>
            </button>
          </div>
        </div>
        
        <h3 class="text-lg font-semibold text-iron-hardware-800 mb-1 group-hover:text-raw-walnut-700 transition-colors">
          {{ wordbook.name }}
        </h3>
        <p v-if="wordbook.description" class="text-iron-hardware-500 text-sm line-clamp-2">
          {{ wordbook.description }}
        </p>
        
        <div class="mt-4 pt-4 border-t border-washed-linen-200 text-xs text-iron-hardware-400">
          {{ t('wordbooks.createdAt') }} {{ new Date(wordbook.created_at).toLocaleDateString() }}
        </div>
      </router-link>
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
        <div v-if="showImportModal" class="fixed inset-0 z-50 flex items-center justify-center">
          <div class="absolute inset-0 bg-black/20 backdrop-blur-sm" @click="closeImportModal" />
          <div class="card relative w-full max-w-md mx-4 animate-modal-in">
            <h2 class="text-xl font-bold text-iron-hardware-800 mb-6">{{ t('importExport.importWordbook') }}</h2>
            
            <div v-if="importResult" class="space-y-4">
              <div class="text-center py-4">
                <div class="w-16 h-16 mx-auto mb-4 rounded-full bg-green-100 flex items-center justify-center">
                  <svg class="w-8 h-8 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
                  </svg>
                </div>
                <h3 class="text-lg font-medium text-iron-hardware-700 mb-2">{{ t('importExport.success') }}</h3>
                <p class="text-iron-hardware-500">
                  {{ t('importExport.chaptersCreated') }}: {{ importResult.chapters_created }}<br>
                  {{ t('importExport.wordsCreated') }}: {{ importResult.words_created }}
                </p>
              </div>
              <button @click="closeImportModal" class="btn-primary w-full">
                {{ t('common.close') }}
              </button>
            </div>
            
            <form v-else @submit.prevent="handleImportWordbook" class="space-y-4">
              <div>
                <label class="block text-sm font-medium text-iron-hardware-700 mb-2">
                  {{ t('importExport.name') }}
                </label>
                <input
                  v-model="importName"
                  type="text"
                  required
                  class="input-field"
                  :placeholder="t('importExport.namePlaceholder')"
                />
              </div>
              
              <div>
                <label class="block text-sm font-medium text-iron-hardware-700 mb-2">
                  {{ t('importExport.selectFile') }}
                </label>
                <input
                  type="file"
                  accept=".json,.xml,.xlsx,.xls,.tsv,.csv"
                  @change="handleFileChange"
                  class="block w-full text-sm text-iron-hardware-500 file:mr-4 file:py-2 file:px-4 file:rounded-lg file:border-0 file:text-sm file:font-medium file:bg-washed-linen-100 file:text-iron-hardware-700 hover:file:bg-washed-linen-200"
                />
              </div>
              
              <div class="pt-2">
                <p class="text-sm text-iron-hardware-500 mb-2">{{ t('importExport.downloadTemplate') }}:</p>
                <div class="flex gap-2">
                  <button type="button" @click="downloadTemplate('json')" class="text-sm text-raw-walnut-600 hover:underline">
                    {{ t('importExport.formats.json') }}
                  </button>
                  <button type="button" @click="downloadTemplate('xml')" class="text-sm text-raw-walnut-600 hover:underline">
                    {{ t('importExport.formats.xml') }}
                  </button>
                  <button type="button" @click="downloadTemplate('xlsx')" class="text-sm text-raw-walnut-600 hover:underline">
                    {{ t('importExport.formats.xlsx') }}
                  </button>
                  <button type="button" @click="downloadTemplate('csv')" class="text-sm text-raw-walnut-600 hover:underline">
                    {{ t('importExport.formats.csv') }}
                  </button>
                </div>
              </div>
              
              <div class="flex gap-3 pt-4">
                <button type="button" @click="closeImportModal" class="btn-secondary flex-1">
                  {{ t('common.cancel') }}
                </button>
                <button type="submit" :disabled="isImporting || !importFile" class="btn-primary flex-1 disabled:opacity-50">
                  {{ isImporting ? t('importExport.importing') : t('importExport.import') }}
                </button>
              </div>
            </form>
          </div>
        </div>
      </Transition>
      
      <Transition
        enter-active-class="transition-all duration-300 ease-out"
        enter-from-class="opacity-0"
        enter-to-class="opacity-100"
        leave-active-class="transition-all duration-200 ease-in"
        leave-from-class="opacity-100"
        leave-to-class="opacity-0"
      >
        <div v-if="showEditModal" class="fixed inset-0 z-50 flex items-center justify-center">
          <div class="absolute inset-0 bg-black/20 backdrop-blur-sm" @click="showEditModal = false" />
          <div class="card relative w-full max-w-md mx-4 animate-modal-in">
            <h2 class="text-xl font-bold text-iron-hardware-800 mb-6">{{ t('wordbooks.modal.editTitle') }}</h2>
            
            <form @submit.prevent="handleEdit" class="space-y-4">
              <div>
                <label for="editName" class="block text-sm font-medium text-iron-hardware-700 mb-2">
                  {{ t('wordbooks.modal.name') }}
                </label>
                <input
                  id="editName"
                  v-model="editName"
                  type="text"
                  required
                  class="input-field"
                  :placeholder="t('wordbooks.modal.namePlaceholder')"
                />
              </div>
              
              <div>
                <label for="editDescription" class="block text-sm font-medium text-iron-hardware-700 mb-2">
                  {{ t('wordbooks.modal.description') }} <span class="text-iron-hardware-400">({{ t('common.optional') }})</span>
                </label>
                <textarea
                  id="editDescription"
                  v-model="editDescription"
                  rows="3"
                  class="input-field resize-none"
                  :placeholder="t('wordbooks.modal.descriptionPlaceholder')"
                />
              </div>
              
              <div class="flex gap-3 pt-4">
                <button type="button" @click="showEditModal = false" class="btn-secondary flex-1">
                  {{ t('common.cancel') }}
                </button>
                <button type="submit" class="btn-primary flex-1">
                  {{ t('common.save') }}
                </button>
              </div>
            </form>
          </div>
        </div>
      </Transition>
      
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
          <Transition
            enter-active-class="transition-all duration-300 ease-out"
            enter-from-class="opacity-0 scale-95 translate-y-4"
            enter-to-class="opacity-100 scale-100 translate-y-0"
            leave-active-class="transition-all duration-200 ease-in"
            leave-from-class="opacity-100 scale-100"
            leave-to-class="opacity-0 scale-95"
          >
            <div v-if="showCreateModal" class="card relative w-full max-w-md mx-4">
              <h2 class="text-xl font-bold text-iron-hardware-800 mb-6">{{ t('wordbooks.modal.createTitle') }}</h2>
              
              <form @submit.prevent="handleCreate" class="space-y-4">
                <div>
                  <label for="name" class="block text-sm font-medium text-iron-hardware-700 mb-2">
                    {{ t('wordbooks.modal.name') }}
                  </label>
                  <input
                    id="name"
                    v-model="newWordbookName"
                    type="text"
                    required
                    class="input-field"
                    :placeholder="t('wordbooks.modal.namePlaceholder')"
                  />
                </div>
                
                <div>
                  <label for="description" class="block text-sm font-medium text-iron-hardware-700 mb-2">
                    {{ t('wordbooks.modal.description') }} <span class="text-iron-hardware-400">({{ t('common.optional') }})</span>
                  </label>
                  <textarea
                    id="description"
                    v-model="newWordbookDescription"
                    rows="3"
                    class="input-field resize-none"
                    :placeholder="t('wordbooks.modal.descriptionPlaceholder')"
                  />
                </div>
                
                <div class="flex gap-3 pt-4">
                  <button type="button" @click="showCreateModal = false" class="btn-secondary flex-1">
                    {{ t('common.cancel') }}
                  </button>
                  <button type="submit" class="btn-primary flex-1">
                    {{ t('wordbooks.modal.create') }}
                  </button>
                </div>
              </form>
            </div>
          </Transition>
        </div>
      </Transition>
    </Teleport>
  </div>
</template>