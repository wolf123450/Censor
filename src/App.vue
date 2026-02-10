<template>
  <div id="app" class="app">
    <header class="header">
      <div class="header-content">
        <h1>🔒 Censor - Content Moderation Tool</h1>
        <div class="header-actions">
          <button @click="showSettings = !showSettings" class="btn-settings">⚙️ Settings</button>
        </div>
      </div>
    </header>

    <main class="main-content">
      <div v-if="!currentProject" class="welcome-screen">
        <h2>Welcome to Censor</h2>
        <p>Upload an image to get started with intelligent content moderation</p>
        <div class="upload-area" @drop="handleDrop" @dragover.prevent @dragleave="dragActive = false">
          <input type="file" ref="fileInput" @change="handleFileSelect" accept="image/*" hidden />
          <button @click="fileInput?.click()" class="btn-upload">
            📁 Select or Drag Image Here
          </button>
        </div>
      </div>

      <div v-else class="editor-screen">
        <div class="editor-container">
          <div class="canvas-section">
            <CanvasEditor 
              :image="currentProject.image_base64"
              :regions="currentProject.regions"
              @add-region="addRegion"
              @update-region="updateRegion"
            />
          </div>

          <div class="sidebar">
            <RegionList 
              :regions="currentProject.regions"
              @select-region="selectRegion"
              @delete-region="deleteRegion"
              @update-method="updateCensoringMethod"
              :selected-region="selectedRegion"
            />
          </div>
        </div>
      </div>
    </main>

    <SettingsModal v-if="showSettings" @close="showSettings = false" />

    <div v-if="loading" class="loading-overlay">
      <div class="spinner">Processing...</div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive } from 'vue'
import CanvasEditor from './components/CanvasEditor.vue'
import RegionList from './components/RegionList.vue'
import SettingsModal from './components/SettingsModal.vue'
import { useImageStore } from './stores/imageStore'
import { useProjectStore } from './stores/projectStore'

const imageStore = useImageStore()
const projectStore = useProjectStore()

const fileInput = ref<HTMLInputElement>()
const dragActive = ref(false)
const loading = ref(false)
const showSettings = ref(false)
const selectedRegion = ref<string | null>(null)

interface Project {
  id: string
  name: string
  image_base64: string
  regions: any[]
  settings: any
}

const currentProject = ref<Project | null>(null)

const handleFileSelect = async (e: Event) => {
  const target = e.target as HTMLInputElement
  if (target.files?.[0]) {
    await loadImage(target.files[0])
  }
}

const handleDrop = async (e: DragEvent) => {
  e.preventDefault()
  dragActive.value = false

  const files = e.dataTransfer?.files
  if (files?.[0]) {
    await loadImage(files[0])
  }
}

const loadImage = async (file: File) => {
  loading.value = true
  try {
    const reader = new FileReader()
    reader.onload = async (e) => {
      const base64 = e.target?.result as string
      
      currentProject.value = {
        id: Math.random().toString(36).substr(2, 9),
        name: file.name,
        image_base64: base64,
        regions: [],
        settings: {
          detect_nudity: true,
          nudity_locale: 'us',
          detect_text_redaction: false,
          target_audience: 'general'
        }
      }

      imageStore.setCurrentImage(base64)
      
      // Automatically detect content
      await detectContent()
    }
    reader.readAsDataURL(file)
  } finally {
    loading.value = false
  }
}

const detectContent = async () => {
  if (!currentProject.value) return
  
  loading.value = true
  try {
    // Call Ollama to detect content
    const regions = await imageStore.detectContent(
      currentProject.value.image_base64,
      'nudity',
      currentProject.value.settings.nudity_locale
    )
    
    if (currentProject.value && regions) {
      currentProject.value.regions = regions
    }
  } catch (error) {
    console.error('Detection failed:', error)
  } finally {
    loading.value = false
  }
}

const addRegion = (region: any) => {
  if (currentProject.value) {
    currentProject.value.regions.push(region)
  }
}

const updateRegion = (updatedRegion: any) => {
  if (currentProject.value) {
    const index = currentProject.value.regions.findIndex(r => r.id === updatedRegion.id)
    if (index >= 0) {
      currentProject.value.regions[index] = updatedRegion
    }
  }
}

const selectRegion = (regionId: string) => {
  selectedRegion.value = regionId
}

const deleteRegion = (regionId: string) => {
  if (currentProject.value) {
    currentProject.value.regions = currentProject.value.regions.filter(r => r.id !== regionId)
  }
}

const updateCensoringMethod = (regionId: string, method: string) => {
  if (currentProject.value) {
    const region = currentProject.value.regions.find(r => r.id === regionId)
    if (region) {
      region.censoring_method = method
    }
  }
}
</script>

<style scoped lang="css">
* {
  box-sizing: border-box;
}

.app {
  display: flex;
  flex-direction: column;
  height: 100vh;
  background: #0f0f0f;
  color: #e0e0e0;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
}

.header {
  background: linear-gradient(135deg, #1a1a1a 0%, #2a2a2a 100%);
  border-bottom: 1px solid #333;
  padding: 1rem 0;
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.5);
}

.header-content {
  max-width: 1600px;
  margin: 0 auto;
  padding: 0 2rem;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.header h1 {
  margin: 0;
  font-size: 1.5rem;
  background: linear-gradient(135deg, #00d4ff, #0099ff);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}

.header-actions {
  display: flex;
  gap: 1rem;
}

.main-content {
  flex: 1;
  overflow: auto;
  display: flex;
  align-items: center;
  justify-content: center;
}

.welcome-screen {
  text-align: center;
  animation: fadeIn 0.3s ease-in;
}

.welcome-screen h2 {
  font-size: 2rem;
  margin: 0 0 1rem 0;
  color: #00d4ff;
}

.welcome-screen p {
  font-size: 1.1rem;
  color: #999;
  margin: 0 0 2rem 0;
}

.upload-area {
  border: 2px dashed #444;
  border-radius: 8px;
  padding: 3rem;
  cursor: pointer;
  transition: all 0.3s ease;
  background: rgba(0, 212, 255, 0.02);
}

.upload-area:hover {
  border-color: #00d4ff;
  background: rgba(0, 212, 255, 0.05);
}

.btn-upload {
  background: linear-gradient(135deg, #00d4ff, #0099ff);
  color: white;
  border: none;
  padding: 1rem 2rem;
  font-size: 1.1rem;
  border-radius: 8px;
  cursor: pointer;
  font-weight: 600;
  transition: all 0.3s ease;
}

.btn-upload:hover {
  transform: translateY(-2px);
  box-shadow: 0 8px 20px rgba(0, 212, 255, 0.3);
}

.editor-screen {
  width: 100%;
  height: 100%;
  display: flex;
}

.editor-container {
  width: 100%;
  display: flex;
  gap: 1rem;
  padding: 1rem;
  max-width: 1600px;
  margin: 0 auto;
}

.canvas-section {
  flex: 1;
  background: #1a1a1a;
  border-radius: 8px;
  overflow: hidden;
  border: 1px solid #333;
}

.sidebar {
  width: 300px;
  background: #1a1a1a;
  border-radius: 8px;
  border: 1px solid #333;
  overflow-y: auto;
}

.btn-settings {
  background: #2a2a2a;
  border: 1px solid #444;
  color: #e0e0e0;
  padding: 0.5rem 1rem;
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.3s ease;
  font-size: 0.9rem;
}

.btn-settings:hover {
  background: #333;
  border-color: #00d4ff;
}

.loading-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.7);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.spinner {
  font-size: 1.5rem;
  color: #00d4ff;
  animation: spin 1s linear infinite;
}

@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translateY(10px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}
</style>
