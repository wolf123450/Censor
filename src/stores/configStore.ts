import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'

export const useConfigStore = defineStore('config', () => {
  const config = ref<any>(null)
  const loading = ref(false)
  const error = ref<string | null>(null)

  const fetchConfig = async () => {
    loading.value = true
    error.value = null

    try {
      const result = await invoke<string>('get_config')
      config.value = JSON.parse(result)
      return config.value
    } catch (err) {
      error.value = String(err)
      console.error('Config fetch error:', err)
      return null
    } finally {
      loading.value = false
    }
  }

  const updateConfig = async (newConfig: any) => {
    loading.value = true
    error.value = null

    try {
      await invoke('set_config', {
        config_json: JSON.stringify(newConfig)
      })
      config.value = newConfig
      return true
    } catch (err) {
      error.value = String(err)
      console.error('Config update error:', err)
      return false
    } finally {
      loading.value = false
    }
  }

  return {
    config,
    loading,
    error,
    fetchConfig,
    updateConfig
  }
})
