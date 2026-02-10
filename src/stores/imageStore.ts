import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'

export const useImageStore = defineStore('image', () => {
  const currentImage = ref<string | null>(null)
  const loading = ref(false)
  const error = ref<string | null>(null)

  const setCurrentImage = (imageBase64: string) => {
    currentImage.value = imageBase64
  }

  const detectContent = async (
    imageBase64: string,
    detectionType: 'nudity' | 'text_redaction',
    locale: string
  ) => {
    loading.value = true
    error.value = null
    
    try {
      // Get config first
      const configJson = await invoke<string>('get_config')
      
      // Call detect_content command
      const result = await invoke<string>('detect_content', {
        image_base64: imageBase64,
        detection_type: detectionType,
        locale: locale,
        config_json: configJson
      })
      
      return JSON.parse(result)
    } catch (err) {
      error.value = String(err)
      console.error('Detection error:', err)
      return []
    } finally {
      loading.value = false
    }
  }

  const applyCensoring = async (
    imageBase64: string,
    regions: any[]
  ) => {
    loading.value = true
    error.value = null
    
    try {
      const result = await invoke<string>('apply_censoring', {
        image_base64: imageBase64,
        regions_json: JSON.stringify(regions)
      })
      
      return result
    } catch (err) {
      error.value = String(err)
      console.error('Censoring error:', err)
      return null
    } finally {
      loading.value = false
    }
  }

  return {
    currentImage,
    loading,
    error,
    setCurrentImage,
    detectContent,
    applyCensoring
  }
})
