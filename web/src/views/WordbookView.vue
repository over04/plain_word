<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { useWordbookStore } from '@/stores/wordbook'

const { t } = useI18n()
const route = useRoute()
const router = useRouter()
const wordbookStore = useWordbookStore()

const wordbookId = computed(() => Number(route.params.id))
const showCreateModal = ref(false)
const newChapterName = ref('')
const editingChapterId = ref<number | null>(null)
const editingChapterName = ref('')
const editingWordbook = ref(false)
const editWordbookName = ref('')
const editWordbookDescription = ref('')
const showImportModal = ref(false)
const showExportMenu = ref(false)
const exportMenuPosition = ref({ top: 0, right: 0 })
const importFile = ref<File | null>(null)
const importName = ref('')
const isImporting = ref(false)
const importResult = ref<{ chapters_created: number; words_created: number } | null>(null)

onMounted(async () => {
  await wordbookStore.fetchWordbook(wordbookId.value)
  await wordbookStore.fetchChapters(wordbookId.value)
})

async function handleCreateChapter() {
  if (!newChapterName.value.trim()) return
  
  await wordbookStore.createChapter(wordbookId.value, {
    name: newChapterName.value
  })
  
  newChapterName.value = ''
  showCreateModal.value = false
}

function startEdit(chapterId: number, name: string) {
  editingChapterId.value = chapterId
  editingChapterName.value = name
}

async function saveEdit() {
  if (!editingChapterId.value || !editingChapterName.value.trim()) return
  
  await wordbookStore.updateChapter(wordbookId.value, editingChapterId.value, {
    name: editingChapterName.value
  })
  
  editingChapterId.value = null
  editingChapterName.value = ''
}

function cancelEdit() {
  editingChapterId.value = null
  editingChapterName.value = ''
}

async function handleDeleteChapter(chapterId: number) {
  if (confirm(t('chapters.deleteConfirm'))) {
    await wordbookStore.deleteChapter(wordbookId.value, chapterId)
  }
}

function startEditWordbook() {
  if (wordbookStore.currentWordbook) {
    editWordbookName.value = wordbookStore.currentWordbook.name
    editWordbookDescription.value = wordbookStore.currentWordbook.description || ''
    editingWordbook.value = true
  }
}

async function saveWordbookEdit() {
  if (!editWordbookName.value.trim()) return
  
  await wordbookStore.updateWordbook(wordbookId.value, {
    name: editWordbookName.value,
    description: editWordbookDescription.value || undefined
  })
  
  editingWordbook.value = false
}

function cancelWordbookEdit() {
  editingWordbook.value = false
}

function handleFileChange(event: Event) {
  const target = event.target as HTMLInputElement
  if (target.files && target.files.length > 0) {
    importFile.value = target.files[0]
  }
}

async function handleImportChapter() {
  if (!importFile.value || !importName.value.trim()) return
  
  isImporting.value = true
  try {
    const formData = new FormData()
    formData.append('file', importFile.value)
    formData.append('name', importName.value)
    
    const response = await fetch(`/api/import/wordbooks/${wordbookId.value}/chapters`, {
      method: 'POST',
      body: formData,
      credentials: 'include'
    })
    
    if (!response.ok) {
      throw new Error('Import failed')
    }
    
    importResult.value = await response.json()
    await wordbookStore.fetchChapters(wordbookId.value)
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
  window.open(`/api/import/templates?format=${format}&target=chapter`, '_blank')
}

function toggleExportMenu(event: Event) {
  const button = event.currentTarget as HTMLElement
  const rect = button.getBoundingClientRect()
  exportMenuPosition.value = {
    top: rect.bottom + 4,
    right: window.innerWidth - rect.right
  }
  showExportMenu.value = !showExportMenu.value
}

function exportWordbook(format: string) {
  window.open(`/api/export/wordbooks/${wordbookId.value}?format=${format}`, '_blank')
  showExportMenu.value = false
}
</script>

<template>
  <div>
    <button 
      @click="router.push('/wordbooks')" 
      class="flex items-center gap-2 text-iron-hardware-500 hover:text-iron-hardware-700 mb-6 
             transition-all duration-300 hover:-translate-x-1"
    >
      <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
      </svg>
      {{ t('chapters.backTo') }} {{ t('nav.wordbooks') }}
    </button>
    
    <div v-if="wordbookStore.currentWordbook" class="mb-8 animate-fade-in-up">
      <div v-if="editingWordbook" class="card space-y-4">
        <div>
          <label class="block text-sm font-medium text-iron-hardware-700 mb-2">{{ t('wordbooks.modal.name') }}</label>
          <input
            v-model="editWordbookName"
            type="text"
            class="input-field"
            @keyup.enter="saveWordbookEdit"
            @keyup.escape="cancelWordbookEdit"
          />
        </div>
        <div>
          <label class="block text-sm font-medium text-iron-hardware-700 mb-2">
            {{ t('wordbooks.modal.description') }} <span class="text-iron-hardware-400">({{ t('common.optional') }})</span>
          </label>
          <textarea
            v-model="editWordbookDescription"
            rows="2"
            class="input-field resize-none"
          />
        </div>
        <div class="flex gap-2">
          <button @click="saveWordbookEdit" class="btn-primary text-sm">{{ t('common.save') }}</button>
          <button @click="cancelWordbookEdit" class="btn-secondary text-sm">{{ t('common.cancel') }}</button>
        </div>
      </div>
      <div v-else class="group">
        <div class="flex items-center gap-3">
          <h1 class="text-3xl font-bold text-iron-hardware-800">
            {{ wordbookStore.currentWordbook.name }}
          </h1>
          <button
            @click="startEditWordbook"
            class="p-2 text-iron-hardware-400 hover:text-iron-hardware-600 opacity-0 group-hover:opacity-100 transition-all duration-300"
            :title="t('common.edit')"
          >
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                    d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z" />
            </svg>
          </button>
        </div>
        <p v-if="wordbookStore.currentWordbook.description" class="text-iron-hardware-500 mt-2">
          {{ wordbookStore.currentWordbook.description }}
        </p>
      </div>
    </div>
    
    <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-3 mb-6 animate-fade-in-up animation-delay-200">
      <h2 class="text-xl font-semibold text-iron-hardware-700">{{ t('chapters.title') }}</h2>
      <div class="flex flex-wrap items-center gap-2">
        <button @click="showImportModal = true" class="btn-secondary text-sm py-1.5 px-3">
          {{ t('importExport.importChapter') }}
        </button>
        <button @click="toggleExportMenu" class="btn-secondary text-sm py-1.5 px-3">
          {{ t('importExport.exportWordbook') }}
        </button>
        <button @click="showCreateModal = true" class="btn-primary text-sm py-1.5 px-3">
          {{ t('chapters.add') }}
        </button>
      </div>
    </div>
    
    <div v-if="wordbookStore.chapters.length === 0" class="card text-center py-12 animate-fade-in-up animation-delay-400">
      <div class="w-16 h-16 mx-auto mb-4 rounded-organic bg-washed-linen-200 flex items-center justify-center animate-float">
        <svg class="w-8 h-8 text-iron-hardware-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
                d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
        </svg>
      </div>
      <h3 class="text-lg font-medium text-iron-hardware-700 mb-2">{{ t('chapters.empty.title') }}</h3>
      <p class="text-iron-hardware-500 mb-4">{{ t('chapters.empty.desc') }}</p>
      <button @click="showCreateModal = true" class="btn-primary text-sm">
        {{ t('chapters.empty.action') }}
      </button>
    </div>
    
    <TransitionGroup 
      v-else 
      tag="div" 
      class="space-y-3"
      enter-active-class="transition-all duration-500 ease-out"
      enter-from-class="opacity-0 translate-x-4"
      enter-to-class="opacity-100 translate-x-0"
      leave-active-class="transition-all duration-300 ease-in"
      leave-from-class="opacity-100"
      leave-to-class="opacity-0 -translate-x-4"
    >
      <div
        v-for="(chapter, index) in wordbookStore.chapters"
        :key="chapter.id"
        class="card flex items-center justify-between group hover:scale-[1.01] transition-all duration-300"
      >
        <div class="flex items-center gap-4 flex-1">
          <div class="w-10 h-10 rounded-xl bg-natural-oak-100 flex items-center justify-center 
                      text-natural-oak-600 font-semibold group-hover:scale-110 transition-transform duration-300">
            {{ index + 1 }}
          </div>
          
          <div v-if="editingChapterId === chapter.id" class="flex items-center gap-2 flex-1">
            <input
              v-model="editingChapterName"
              type="text"
              class="input-field py-2"
              @keyup.enter="saveEdit"
              @keyup.escape="cancelEdit"
            />
            <button @click="saveEdit" class="text-green-600 hover:text-green-700 transition-colors">
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
              </svg>
            </button>
            <button @click="cancelEdit" class="text-iron-hardware-400 hover:text-iron-hardware-600 transition-colors">
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
              </svg>
            </button>
          </div>
          
          <router-link
            v-else
            :to="`/wordbook/${wordbookId}/chapter/${chapter.id}`"
            class="flex-1 group-hover:text-raw-walnut-600 transition-colors"
          >
            <h3 class="font-medium text-iron-hardware-800">{{ chapter.name }}</h3>
          </router-link>
        </div>
        
        <div class="flex items-center gap-2 opacity-0 group-hover:opacity-100 transition-all duration-300">
          <button
            @click="startEdit(chapter.id, chapter.name)"
            class="p-2 text-iron-hardware-400 hover:text-iron-hardware-600 hover:scale-110 transition-all duration-200"
          >
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
                    d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z" />
            </svg>
          </button>
          <button
            @click="handleDeleteChapter(chapter.id)"
            class="p-2 text-iron-hardware-400 hover:text-red-500 hover:scale-110 transition-all duration-200"
          >
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
                    d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
            </svg>
          </button>
          <router-link
            :to="`/wordbook/${wordbookId}/chapter/${chapter.id}`"
            class="p-2 text-iron-hardware-400 hover:text-raw-walnut-600 hover:translate-x-1 transition-all duration-200"
          >
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
            </svg>
          </router-link>
        </div>
      </div>
    </TransitionGroup>
    
    <Teleport to="body">
      <Transition
        enter-active-class="transition-all duration-200 ease-out"
        enter-from-class="opacity-0 scale-95"
        enter-to-class="opacity-100 scale-100"
        leave-active-class="transition-all duration-150 ease-in"
        leave-from-class="opacity-100 scale-100"
        leave-to-class="opacity-0 scale-95"
      >
        <div
          v-if="showExportMenu"
          class="fixed w-32 bg-white rounded-xl shadow-soft-lg border border-washed-linen-200 py-2 z-[100]"
          :style="{ top: exportMenuPosition.top + 'px', right: exportMenuPosition.right + 'px' }"
        >
          <button @click="exportWordbook('json')" class="w-full px-3 py-2 text-left text-sm hover:bg-washed-linen-100 transition-colors">
            {{ t('importExport.formats.json') }}
          </button>
          <button @click="exportWordbook('xml')" class="w-full px-3 py-2 text-left text-sm hover:bg-washed-linen-100 transition-colors">
            {{ t('importExport.formats.xml') }}
          </button>
          <button @click="exportWordbook('xlsx')" class="w-full px-3 py-2 text-left text-sm hover:bg-washed-linen-100 transition-colors">
            {{ t('importExport.formats.xlsx') }}
          </button>
          <button @click="exportWordbook('csv')" class="w-full px-3 py-2 text-left text-sm hover:bg-washed-linen-100 transition-colors">
            {{ t('importExport.formats.csv') }}
          </button>
        </div>
      </Transition>
      
      <div v-if="showExportMenu" class="fixed inset-0 z-[99]" @click="showExportMenu = false" />
      
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
            <h2 class="text-xl font-bold text-iron-hardware-800 mb-6">{{ t('importExport.importChapter') }}</h2>
            
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
            
            <form v-else @submit.prevent="handleImportChapter" class="space-y-4">
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
        <div v-if="showCreateModal" class="fixed inset-0 z-50 flex items-center justify-center">
          <div class="absolute inset-0 bg-black/20 backdrop-blur-sm" @click="showCreateModal = false" />
          <div class="card relative w-full max-w-md mx-4 animate-modal-in">
            <h2 class="text-xl font-bold text-iron-hardware-800 mb-6">{{ t('chapters.modal.createTitle') }}</h2>
            
            <form @submit.prevent="handleCreateChapter" class="space-y-4">
              <div>
                <label for="chapterName" class="block text-sm font-medium text-iron-hardware-700 mb-2">
                  {{ t('chapters.modal.name') }}
                </label>
                <input
                  id="chapterName"
                  v-model="newChapterName"
                  type="text"
                  required
                  class="input-field"
                  :placeholder="t('chapters.modal.namePlaceholder')"
                />
              </div>
              
              <div class="flex gap-3 pt-4">
                <button type="button" @click="showCreateModal = false" class="btn-secondary flex-1">
                  {{ t('common.cancel') }}
                </button>
                <button type="submit" class="btn-primary flex-1">
                  {{ t('chapters.modal.create') }}
                </button>
              </div>
            </form>
          </div>
        </div>
      </Transition>
    </Teleport>
  </div>
</template>

<style scoped>
.animation-delay-200 {
  animation-delay: 0.2s;
}
.animation-delay-400 {
  animation-delay: 0.4s;
}
</style>