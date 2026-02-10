<template>
  <div class="canvas-editor">
    <div class="canvas-wrapper" @mousedown="startDrawing" @mousemove="draw" @mouseup="stopDrawing" @mouseleave="stopDrawing">
      <canvas 
        ref="canvas"
        class="main-canvas"
        :width="canvasWidth"
        :height="canvasHeight"
      />
      <svg v-if="showPolygons" class="overlay-svg">
        <g v-for="region in regions" :key="region.id" :class="{selected: region.id === selectedRegionId}">
          <!-- Polygon outline -->
          <polygon 
            :points="polygonPoints(region.polygon)" 
            class="region-polygon"
            :style="{stroke: getColorForMethod(region.censoring_method)}"
          />
          <!-- Control points -->
          <circle
            v-for="(point, idx) in region.polygon"
            :key="`${region.id}-${idx}`"
            :cx="point[0]"
            :cy="point[1]"
            r="5"
            class="control-point"
            :class="{active: draggingPoint?.regionId === region.id && draggingPoint?.pointIdx === idx}"
            @mousedown.stop="startDraggingPoint(region.id, idx)"
          />
        </g>
      </svg>
    </div>
    <div class="canvas-controls">
      <button @click="togglePolygonView" class="btn-control">{{ showPolygons ? '👁️ Hide' : '👁️ Show' }} Masks</button>
      <button @click="drawNewMaskMode" :class="{active: drawingMode}" class="btn-control">✏️ Draw Mask</button>
      <button @click="exportImage" class="btn-control">💾 Export</button>
      <button @click="applyEffects" class="btn-control btn-primary">✓ Apply Effects</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, onMounted, computed } from 'vue'
import { v4 as uuidv4 } from 'uuid'

interface Region {
  id: string
  polygon: [number, number][]
  censoring_method: string
}

const props = defineProps({
  image: String,
  regions: {
    type: Array as () => Region[],
    default: () => []
  }
})

const emit = defineEmits<{
  'add-region': [region: Region]
  'update-region': [region: Region]
}>()

const canvas = ref<HTMLCanvasElement>()
const ctx = ref<CanvasRenderingContext2D | null>(null)
const showPolygons = ref(true)
const drawingMode = ref(false)
const currentPath = ref<[number, number][]>([])

const canvasWidth = ref(0)
const canvasHeight = ref(0)
const selectedRegionId = ref<string | null>(null)

interface DraggingPoint {
  regionId: string
  pointIdx: number
}

const draggingPoint = ref<DraggingPoint | null>(null)

const polygonPoints = (polygon: [number, number][]) => {
  return polygon.map(p => `${p[0]},${p[1]}`).join(' ')
}

const getColorForMethod = (method: string) => {
  const colors: Record<string, string> = {
    'blackout': '#000000',
    'blur': '#0099ff',
    'pixelate': '#00d4ff'
  }
  return colors[method] || '#00d4ff'
}

onMounted(() => {
  if (canvas.value) {
    ctx.value = canvas.value.getContext('2d')
  }
  redraw()
})

watch(() => props.image, () => {
  redraw()
}, { immediate: true })

watch(() => props.regions, () => {
  redraw()
})

const redraw = () => {
  if (!canvas.value || !ctx.value || !props.image) return

  const img = new Image()
  img.onload = () => {
    canvas.value!.width = img.width
    canvas.value!.height = img.height
    canvasWidth.value = img.width
    canvasHeight.value = img.height

    ctx.value!.clearRect(0, 0, canvas.value!.width, canvas.value!.height)
    ctx.value!.drawImage(img, 0, 0)
  }
  img.src = props.image || ''
}

const startDrawing = (e: MouseEvent) => {
  if (!drawingMode.value || !canvas.value) return

  const rect = canvas.value.getBoundingClientRect()
  const x = e.clientX - rect.left
  const y = e.clientY - rect.top

  currentPath.value.push([x, y])

  // Draw point
  if (ctx.value) {
    ctx.value.fillStyle = '#00d4ff'
    ctx.value.beginPath()
    ctx.value.arc(x, y, 3, 0, Math.PI * 2)
    ctx.value.fill()
  }
}

const draw = (e: MouseEvent) => {
  if (!drawingMode.value || !canvas.value || currentPath.value.length === 0) return

  const rect = canvas.value.getBoundingClientRect()
  const x = e.clientX - rect.left
  const y = e.clientY - rect.top

  // Draw line preview
  if (ctx.value) {
    redraw()
    ctx.value.strokeStyle = '#00d4ff'
    ctx.value.lineWidth = 2
    ctx.value.beginPath()
    ctx.value.moveTo(currentPath.value[currentPath.value.length - 1][0], currentPath.value[currentPath.value.length - 1][1])
    ctx.value.lineTo(x, y)
    ctx.value.stroke()
  }
}

const stopDrawing = (e: KeyboardEvent | MouseEvent) => {
  // Handle Escape to finish drawing
  if (e instanceof KeyboardEvent && e.key === 'Escape' && currentPath.value.length >= 3) {
    finishDrawing()
  }
}

const finishDrawing = () => {
  if (currentPath.value.length >= 3) {
    const region: Region = {
      id: uuidv4(),
      polygon: currentPath.value,
      censoring_method: 'blur'
    }
    emit('add-region', region)
    currentPath.value = []
    drawingMode.value = false
    redraw()
  }
}

const startDraggingPoint = (regionId: string, pointIdx: number) => {
  selectedRegionId.value = regionId
  draggingPoint.value = { regionId, pointIdx }
}

const togglePolygonView = () => {
  showPolygons.value = !showPolygons.value
}

const drawNewMaskMode = () => {
  drawingMode.value = !drawingMode.value
  if (drawingMode.value) {
    currentPath.value = []
  }
}

const exportImage = async () => {
  if (!canvas.value) return
  const link = document.createElement('a')
  link.href = canvas.value.toDataURL('image/png')
  link.download = 'censored-image.png'
  link.click()
}

const applyEffects = async () => {
  // Call backend to apply censoring
  console.log('Applying effects to regions:', props.regions)
}
</script>

<style scoped lang="css">
.canvas-editor {
  display: flex;
  flex-direction: column;
  height: 100%;
  gap: 1rem;
  padding: 1rem;
}

.canvas-wrapper {
  flex: 1;
  position: relative;
  background: #000;
  border-radius: 8px;
  overflow: hidden;
  display: flex;
  align-items: center;
  justify-content: center;
}

.main-canvas {
  max-width: 100%;
  max-height: 100%;
  cursor: crosshair;
}

.overlay-svg {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  pointer-events: auto;
}

.region-polygon {
  fill: none;
  stroke-width: 2;
  opacity: 0.7;
  transition: opacity 0.2s;
}

.region-polygon.selected {
  opacity: 1;
  stroke-width: 3;
}

.control-point {
  fill: #00d4ff;
  cursor: pointer;
  opacity: 0.6;
  transition: all 0.2s;
}

.control-point:hover,
.control-point.active {
  r: 7;
  opacity: 1;
  filter: drop-shadow(0 0 5px #00d4ff);
}

.canvas-controls {
  display: flex;
  gap: 0.5rem;
  flex-wrap: wrap;
}

.btn-control {
  background: #2a2a2a;
  border: 1px solid #444;
  color: #e0e0e0;
  padding: 0.6rem 1rem;
  border-radius: 6px;
  font-size: 0.9rem;
  transition: all 0.3s ease;
}

.btn-control:hover {
  background: #333;
  border-color: #00d4ff;
  color: #00d4ff;
}

.btn-control.active {
  background: #00d4ff;
  border-color: #00d4ff;
  color: #000;
}

.btn-primary {
  background: linear-gradient(135deg, #00d4ff, #0099ff);
  border: none;
  color: white;
  font-weight: 600;
}

.btn-primary:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 212, 255, 0.3);
}
</style>
