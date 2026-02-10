// Utility functions for image processing
export const downloadImage = (dataUrl: string, filename: string) => {
  const link = document.createElement('a')
  link.href = dataUrl
  link.download = filename
  link.click()
}

export const fileToBase64 = (file: File): Promise<string> => {
  return new Promise((resolve, reject) => {
    const reader = new FileReader()
    reader.onload = () => resolve(reader.result as string)
    reader.onerror = reject
    reader.readAsDataURL(file)
  })
}

export const base64ToImage = async (base64Data: string): Promise<HTMLImageElement> => {
  return new Promise((resolve, reject) => {
    const img = new Image()
    img.onload = () => resolve(img)
    img.onerror = reject
    img.src = base64Data
  })
}

export const drawPolygonOnCanvas = (
  ctx: CanvasRenderingContext2D,
  polygon: [number, number][],
  color: string = '#00d4ff',
  filled: boolean = false
) => {
  if (polygon.length < 2) return

  ctx.strokeStyle = color
  ctx.fillStyle = color
  ctx.lineWidth = 2

  ctx.beginPath()
  ctx.moveTo(polygon[0][0], polygon[0][1])

  for (let i = 1; i < polygon.length; i++) {
    ctx.lineTo(polygon[i][0], polygon[i][1])
  }

  ctx.closePath()

  if (filled) {
    ctx.fill()
  } else {
    ctx.stroke()
  }
}

export const calculatePolygonArea = (polygon: [number, number][]): number => {
  let area = 0
  for (let i = 0; i < polygon.length; i++) {
    const j = (i + 1) % polygon.length
    area += polygon[i][0] * polygon[j][1]
    area -= polygon[j][0] * polygon[i][1]
  }
  return Math.abs(area / 2)
}

export const calculateBoundingBox = (polygon: [number, number][]) => {
  let minX = Infinity,
    maxX = -Infinity,
    minY = Infinity,
    maxY = -Infinity

  for (const [x, y] of polygon) {
    minX = Math.min(minX, x)
    maxX = Math.max(maxX, x)
    minY = Math.min(minY, y)
    maxY = Math.max(maxY, y)
  }

  return { x: minX, y: minY, width: maxX - minX, height: maxY - minY }
}

export const simplifyPolygon = (polygon: [number, number][], epsilon: number = 1.0): [number, number][] => {
  // Ramer-Douglas-Peucker algorithm
  if (polygon.length <= 2) return polygon

  let maxDist = 0
  let maxIdx = 0

  const dmax = (start: [number, number], end: [number, number], points: [number, number][]) => {
    for (let i = 1; i < points.length; i++) {
      const d = pointLineDistance(points[i], start, end)
      if (d > maxDist) {
        maxDist = d
        maxIdx = i
      }
    }
  }

  dmax(polygon[0], polygon[polygon.length - 1], polygon.slice(1, -1))

  if (maxDist > epsilon) {
    const left: [number, number][] = simplifyPolygon(polygon.slice(0, maxIdx + 1), epsilon)
    const right: [number, number][] = simplifyPolygon(polygon.slice(maxIdx), epsilon)
    return [...left.slice(0, -1), ...right]
  } else {
    return [polygon[0], polygon[polygon.length - 1]]
  }
}

const pointLineDistance = (point: [number, number], start: [number, number], end: [number, number]) => {
  const dx = end[0] - start[0]
  const dy = end[1] - start[1]
  const num = Math.abs(dy * point[0] - dx * point[1] + end[0] * start[1] - end[1] * start[0])
  const den = Math.sqrt(dx * dx + dy * dy)
  return num / den
}
