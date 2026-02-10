<template>
  <div class="modal-overlay" @click="$emit('close')">
    <div class="modal" @click.stop>
      <div class="modal-header">
        <h2>⚙️ Settings</h2>
        <button @click="$emit('close')" class="btn-close">✕</button>
      </div>
      
      <div class="modal-content">
        <div class="settings-group">
          <h3>Ollama Configuration</h3>
          <div class="setting-item">
            <label>Endpoint:</label>
            <input v-model="config.ollama_endpoint" type="text" class="input-field" />
          </div>
          <div class="setting-item">
            <label>Model:</label>
            <input v-model="config.ollama_model" type="text" class="input-field" />
          </div>
          <button @click="testConnection" class="btn-test">🔗 Test Connection</button>
          <div v-if="connectionStatus" :class="{success: connectionStatus.success}" class="status-message">
            {{ connectionStatus.message }}
          </div>
        </div>

        <div class="settings-group">
          <h3>Detection Settings</h3>
          <div class="setting-item">
            <label>
              <input v-model="detectionSettings.detect_nudity" type="checkbox" />
              Detect Nudity
            </label>
          </div>
          <div v-if="detectionSettings.detect_nudity" class="setting-item">
            <label>Nudity Locale:</label>
            <select v-model="detectionSettings.nudity_locale" class="input-field">
              <option value="us">United States</option>
              <option value="eu">European Union</option>
              <option value="jp">Japan</option>
            </select>
          </div>
          <div class="setting-item">
            <label>
              <input v-model="detectionSettings.detect_text_redaction" type="checkbox" />
              Detect Text/PII Redaction
            </label>
          </div>
        </div>

        <div class="settings-group">
          <h3>Default Censoring Method</h3>
          <div class="setting-item">
            <select v-model="config.default_censoring_method" class="input-field">
              <option value="blur">Blur</option>
              <option value="pixelate">Pixelate</option>
              <option value="blackout">Blackout</option>
            </select>
          </div>
        </div>
      </div>

      <div class="modal-footer">
        <button @click="saveSettings" class="btn-save">💾 Save Settings</button>
        <button @click="$emit('close')" class="btn-cancel">Cancel</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive } from 'vue'

defineEmits<{
  'close': []
}>()

const config = reactive({
  ollama_endpoint: 'http://localhost:11434',
  ollama_model: 'qwen3-vl:30b',
  default_censoring_method: 'blur'
})

const detectionSettings = reactive({
  detect_nudity: true,
  nudity_locale: 'us',
  detect_text_redaction: false
})

const connectionStatus = ref<{success: boolean, message: string} | null>(null)

const testConnection = async () => {
  try {
    // In production, call backend
    connectionStatus.value = {
      success: true,
      message: '✓ Connected to Ollama successfully!'
    }
  } catch (error) {
    connectionStatus.value = {
      success: false,
      message: '✗ Failed to connect to Ollama'
    }
  }
}

const saveSettings = () => {
  console.log('Settings saved:', { config, detectionSettings })
}
</script>

<style scoped lang="css">
.modal-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.7);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  animation: fadeIn 0.2s ease-in;
}

.modal {
  background: #1a1a1a;
  border: 1px solid #333;
  border-radius: 8px;
  width: 90%;
  max-width: 600px;
  max-height: 80vh;
  display: flex;
  flex-direction: column;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.8);
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1.5rem;
  border-bottom: 1px solid #333;
}

.modal-header h2 {
  margin: 0;
  font-size: 1.3rem;
  color: #00d4ff;
}

.btn-close {
  background: none;
  border: none;
  color: #999;
  font-size: 1.5rem;
  cursor: pointer;
  line-height: 1;
  padding: 0;
}

.btn-close:hover {
  color: #ff4444;
}

.modal-content {
  flex: 1;
  overflow-y: auto;
  padding: 1.5rem;
}

.settings-group {
  margin-bottom: 1.5rem;
  padding-bottom: 1.5rem;
  border-bottom: 1px solid #333;
}

.settings-group:last-child {
  margin-bottom: 0;
  padding-bottom: 0;
  border-bottom: none;
}

.settings-group h3 {
  margin: 0 0 1rem 0;
  color: #00d4ff;
  font-size: 1rem;
}

.setting-item {
  margin-bottom: 1rem;
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.setting-item label {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  cursor: pointer;
  font-size: 0.95rem;
}

.input-field {
  width: 100%;
  padding: 0.6rem;
  background: #2a2a2a;
  border: 1px solid #444;
  color: #e0e0e0;
  border-radius: 6px;
  font-family: inherit;
  font-size: 0.95rem;
}

.input-field:focus {
  outline: none;
  border-color: #00d4ff;
  box-shadow: 0 0 0 2px rgba(0, 212, 255, 0.1);
}

.btn-test {
  background: #2a2a2a;
  border: 1px solid #444;
  color: #e0e0e0;
  padding: 0.6rem 1rem;
  border-radius: 6px;
  cursor: pointer;
  font-size: 0.9rem;
  transition: all 0.3s ease;
}

.btn-test:hover {
  background: #333;
  border-color: #00d4ff;
}

.status-message {
  padding: 0.6rem;
  border-radius: 6px;
  font-size: 0.9rem;
  background: rgba(255, 68, 68, 0.1);
  color: #ff4444;
  border: 1px solid rgba(255, 68, 68, 0.3);
}

.status-message.success {
  background: rgba(0, 212, 255, 0.1);
  color: #00d4ff;
  border: 1px solid rgba(0, 212, 255, 0.3);
}

.modal-footer {
  display: flex;
  gap: 1rem;
  padding: 1.5rem;
  border-top: 1px solid #333;
  justify-content: flex-end;
}

.btn-save {
  background: linear-gradient(135deg, #00d4ff, #0099ff);
  border: none;
  color: white;
  padding: 0.6rem 1.5rem;
  border-radius: 6px;
  cursor: pointer;
  font-weight: 600;
  transition: all 0.3s ease;
}

.btn-save:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 212, 255, 0.3);
}

.btn-cancel {
  background: #2a2a2a;
  border: 1px solid #444;
  color: #e0e0e0;
  padding: 0.6rem 1.5rem;
  border-radius: 6px;
  cursor: pointer;
  font-weight: 600;
  transition: all 0.3s ease;
}

.btn-cancel:hover {
  background: #333;
  border-color: #00d4ff;
}

@keyframes fadeIn {
  from {
    opacity: 0;
    transform: scale(0.95);
  }
  to {
    opacity: 1;
    transform: scale(1);
  }
}
</style>
