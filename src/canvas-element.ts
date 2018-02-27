import { Canvas } from '../native'
import { Context2D } from './context-2d'

export type CanvasCtxType = '2d' | 'webgl' | 'webgl2' | 'bitmaprenderer'

export class CanvasElement {

  private nativeCanvas: Canvas
  private ctx!: Context2D

  constructor(width = 300, height = 150) {
    this.nativeCanvas = new Canvas(width, height)
  }

  getContext(ctxType: CanvasCtxType) {
    switch (ctxType) {
      case '2d':
        const ctx = new Context2D(this)
        this.ctx = ctx
        return ctx
      default:
        console.warn('not implement')
    }
  }

  toBuffer() {
    return this.nativeCanvas.toBuffer([...this.ctx.actions])
  }
}
