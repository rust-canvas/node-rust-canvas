import { CanvasElement } from './canvas-element'
import { CanvasGradient } from './canvas-gradient'
import { CanvasPattern } from './canvas-pattern'
import { ImageData } from './image-data'
import * as Types from './interface'

export type GlobalCompositeOperationType = 'source-over' | 'source-in' | 'source-out' | 'source-atop' |
  'destination-over' | 'destination-in' | 'destination-out' | 'destination-atop' | 'lighter' | 'copy' |
  'xor' | 'multiply' | 'screen' | 'overlay' | 'darken' | 'lighten' | 'color-dodge' | 'color-burn' |
  'hard-light' | 'soft-light' | 'difference' | 'exclusion' | 'hue' | 'saturation' | 'color' | 'luminosity'

export class Context2D {
  fillStyle!: string | CanvasGradient | CanvasPattern
  font!: string
  globalAlpha!: number
  globalCompositeOperation!: GlobalCompositeOperationType
  lineCap!: 'butt' | 'round' | 'square'
  lineDashOffset!: number
  lineJoin!: 'bevel' | 'round' | 'miter'
  lineWidth!: number
  miterLimit!: number
  shadowBlur!: number
  shadowColor!: string
  shadowOffsetX!: number
  shadowOffsetY!: number
  strokeStyle!: string | CanvasGradient | CanvasPattern
  textAlign!: 'left' | 'right' | 'center' | 'start' | 'end'
  textBaseline!: 'top' | 'hanging' | 'middle' | 'alphabetic' | 'ideographic' | 'bottom'

  private state = {
    fillStyle: '#000',
    font: '10px sans-serif',
    globalAlpha: 1.0,
    globalCompositeOperation: 'source-over',
    lineCap: 'butt',
    lineDashOffset: 0.0,
    lineJoin: 'bevel',
    lineWidth: 1.0,
    miterLimit: 10.0,
    shadowBlur: 0,
    shadowColor: '#000',
    shadowOffsetX: 0,
    shadowOffsetY: 0,
    strokeStyle: '#000',
    textAlign: 'start',
    textBaseline: 'alphabetic',
  }

  private actions: Types.Action[] = []

  constructor(public canvas: CanvasElement) {
    Object.assign(this, this.state)
  }

  arc(x: number, y: number, radius: number, startAngle: number, endAngle: number, anticlockwise = false ) {
    this.actions.push({
      type: 'ARC', x, y, radius, startAngle, endAngle, anticlockwise,
    })
  }

  arcTo(x1: number, y1: number, x2: number, y2: number, radius: number) {
    this.actions.push({
      type: 'ARCTO', x1, y1, x2, y2, radius,
    })
  }

  beginPath() {
    this.actions.push({
      type: 'BEGINPATH'
    })
  }

  bezierCurveTo(cp1x: number, cp1y: number, cp2x: number, cp2y: number, x: number, y: number) {
    this.actions.push({
      type: 'BEZIERCURVETO', cp1x, cp1y, cp2x, cp2y, x, y,
    })
  }

  clearRect(x: number, y: number, width: number, height: number) {
    this.actions.push({
      type: 'CLEARRECT', x, y, width, height,
    })
  }

  clip(fillRule?: 'nonzero' | 'evenodd') {
    this.actions.push({
      type: 'CLIP', fillRule,
    })
  }

  closePath() {
    this.actions.push({
      type: 'CLOSEPATH'
    })
  }

  createLinearGradient(x0: number, y0: number, x1: number, y1: number) {
    this.actions.push({
      type: 'CREATELINEARGRADIENT', x0, y0, x1, y1,
    })
  }

  createImageData(imageData: ImageData): ImageData
  createImageData(widht: number, height: number): ImageData
  createImageData(widthOrImageData: number | ImageData, height?: number): ImageData {
    if (!widthOrImageData || !height) {
      throw new RangeError()
    }
    let imageData: ImageData
    if (typeof widthOrImageData === 'number') {
      imageData = new ImageData(widthOrImageData, height!)
    } else {
      imageData = widthOrImageData.clone()
    }
    this.actions.push({
      type: 'CREATEIMAGEDATA',
      imageData,
    })
    return imageData
  }
}
