import { invoke } from '@tauri-apps/api/tauri'

export const censorApi = {
  async processImage(imagePath: string) {
    return invoke('process_image', { image_path: imagePath })
  },

  async detectContent(imageBase64: string, detectionType: string, locale: string, configJson: string) {
    return invoke('detect_content', {
      image_base64: imageBase64,
      detection_type: detectionType,
      locale,
      config_json: configJson
    })
  },

  async applyCensoring(imageBase64: string, regionsJson: string) {
    return invoke('apply_censoring', {
      image_base64: imageBase64,
      regions_json: regionsJson
    })
  },

  async getCensoringRegions(projectJson: string) {
    return invoke('get_censoring_regions', {
      project_json: projectJson
    })
  },

  async adjustRegion(regionJson: string, newPolygonJson: string) {
    return invoke('adjust_region', {
      region_json: regionJson,
      new_polygon_json: newPolygonJson
    })
  },

  async saveProject(projectJson: string, savePath: string) {
    return invoke('save_project', {
      project_json: projectJson,
      save_path: savePath
    })
  },

  async loadProject(projectPath: string) {
    return invoke('load_project', {
      project_path: projectPath
    })
  },

  async getConfig() {
    return invoke('get_config')
  },

  async setConfig(configJson: string) {
    return invoke('set_config', {
      config_json: configJson
    })
  }
}
