<template>
  <div class="region-list">
    <h3>Censoring Regions</h3>
    <div class="regions-container">
      <div 
        v-for="region in regions"
        :key="region.id"
        class="region-item"
        :class="{selected: region.id === selectedRegion}"
        @click="$emit('select-region', region.id)"
      >
        <div class="region-header">
          <span class="region-label">{{ getLabel(region) }}</span>
          <button @click.stop="$emit('delete-region', region.id)" class="btn-delete">✕</button>
        </div>
        <div class="region-controls">
          <label>Method:</label>
          <select 
            :value="region.censoring_method"
            @change="$emit('update-method', region.id, ($event.target as HTMLSelectElement).value)"
            class="method-select"
          >
            <option value="blur">Blur</option>
            <option value="pixelate">Pixelate</option>
            <option value="blackout">Blackout</option>
          </select>
        </div>
        <div class="region-meta">
          <span v-if="region.confidence" class="confidence">
            Confidence: {{ (region.confidence * 100).toFixed(0) }}%
          </span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
interface Region {
  id: string
  detection_type?: string
  censoring_method: string
  confidence?: number
  metadata?: Record<string, string>
}

withDefaults(defineProps<{
  regions: Region[]
  selectedRegion?: string | null
}>(), {
  regions: () => [],
  selectedRegion: null
})

defineEmits<{
  'select-region': [regionId: string]
  'delete-region': [regionId: string]
  'update-method': [regionId: string, method: string]
}>()

const getLabel = (region: Region) => {
  if (region.metadata?.label) {
    return region.metadata.label
  }
  return region.detection_type || 'Region'
}
</script>

<style scoped lang="css">
.region-list {
  display: flex;
  flex-direction: column;
  height: 100%;
  padding: 1rem;
}

.region-list h3 {
  margin: 0 0 1rem 0;
  font-size: 1rem;
  color: #00d4ff;
}

.regions-container {
  flex: 1;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.region-item {
  background: #2a2a2a;
  border: 1px solid #444;
  border-radius: 6px;
  padding: 0.75rem;
  cursor: pointer;
  transition: all 0.3s ease;
}

.region-item:hover {
  background: #333;
  border-color: #00d4ff;
}

.region-item.selected {
  background: #333;
  border-color: #00d4ff;
  box-shadow: 0 0 8px rgba(0, 212, 255, 0.2);
}

.region-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 0.5rem;
}

.region-label {
  font-weight: 600;
  color: #e0e0e0;
  font-size: 0.9rem;
}

.btn-delete {
  background: none;
  border: none;
  color: #999;
  cursor: pointer;
  font-size: 1rem;
  line-height: 1;
  padding: 0;
}

.btn-delete:hover {
  color: #ff4444;
}

.region-controls {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
  margin-bottom: 0.5rem;
}

.region-controls label {
  font-size: 0.8rem;
  color: #999;
}

.method-select {
  width: 100%;
  padding: 0.4rem;
  background: #1a1a1a;
  border: 1px solid #444;
  color: #e0e0e0;
  border-radius: 4px;
  font-size: 0.85rem;
}

.method-select:focus {
  outline: none;
  border-color: #00d4ff;
}

.region-meta {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
  font-size: 0.8rem;
  color: #999;
}

.confidence {
  background: rgba(0, 212, 255, 0.1);
  padding: 0.25rem 0.5rem;
  border-radius: 3px;
}
</style>
