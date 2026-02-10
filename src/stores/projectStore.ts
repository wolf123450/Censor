import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'

interface CensoringProject {
  id: string
  name: string
  image_path: string
  image_base64?: string
  regions: any[]
  created_at: string
  modified_at: string
  settings: any
}

export const useProjectStore = defineStore('project', () => {
  const currentProject = ref<CensoringProject | null>(null)
  const projects = ref<CensoringProject[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)

  const createProject = (name: string, imageBase64: string) => {
    const now = new Date().toISOString()
    const project: CensoringProject = {
      id: Math.random().toString(36).substr(2, 9),
      name,
      image_path: '',
      image_base64: imageBase64,
      regions: [],
      created_at: now,
      modified_at: now,
      settings: {
        detect_nudity: true,
        nudity_locale: 'us',
        detect_text_redaction: false,
        target_audience: 'general'
      }
    }
    
    currentProject.value = project
    projects.value.push(project)
    return project
  }

  const saveProject = async (forcePath?: string) => {
    if (!currentProject.value) return

    loading.value = true
    error.value = null

    try {
      const result = await invoke<string>('save_project', {
        project_json: JSON.stringify(currentProject.value),
        save_path: forcePath || currentProject.value.name
      })
      
      console.log('Project saved:', result)
      return result
    } catch (err) {
      error.value = String(err)
      console.error('Save error:', err)
      return null
    } finally {
      loading.value = false
    }
  }

  const loadProject = async (projectPath: string) => {
    loading.value = true
    error.value = null

    try {
      const result = await invoke<string>('load_project', {
        project_path: projectPath
      })
      
      const project = JSON.parse(result)
      currentProject.value = project
      projects.value.push(project)
      return project
    } catch (err) {
      error.value = String(err)
      console.error('Load error:', err)
      return null
    } finally {
      loading.value = false
    }
  }

  const updateProjectSettings = (settings: any) => {
    if (currentProject.value) {
      currentProject.value.settings = settings
      currentProject.value.modified_at = new Date().toISOString()
    }
  }

  const addRegion = (region: any) => {
    if (currentProject.value) {
      currentProject.value.regions.push(region)
      currentProject.value.modified_at = new Date().toISOString()
    }
  }

  const updateRegion = (regionId: string, updates: any) => {
    if (currentProject.value) {
      const region = currentProject.value.regions.find(r => r.id === regionId)
      if (region) {
        Object.assign(region, updates)
        currentProject.value.modified_at = new Date().toISOString()
      }
    }
  }

  const deleteRegion = (regionId: string) => {
    if (currentProject.value) {
      currentProject.value.regions = currentProject.value.regions.filter(
        r => r.id !== regionId
      )
      currentProject.value.modified_at = new Date().toISOString()
    }
  }

  return {
    currentProject,
    projects,
    loading,
    error,
    createProject,
    saveProject,
    loadProject,
    updateProjectSettings,
    addRegion,
    updateRegion,
    deleteRegion
  }
})
