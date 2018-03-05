import { Canvas } from '../index'
import { Context2D } from './context-2d'
import jpeg = require('jpeg-js')
import UPNG = require('upng-js')

export type CanvasCtxType = '2d' | 'webgl' | 'webgl2' | 'bitmaprenderer'

export class CanvasElement {

  private nativeCanvas: Canvas
  private ctx!: Context2D

  constructor(public width = 300, public height = 150) {
    this.nativeCanvas = new Canvas(width, height)
  }

  getContext(ctxType: CanvasCtxType) {
    switch (ctxType) {
      case '2d':
        if (this.ctx) {
          return this.ctx
        }
        const ctx = new Context2D(this)
        this.ctx = ctx
        return ctx
      default:
        console.warn('not implement')
    }
  }

  toBuffer(type?: string, encoderOptions?: number) {
    return new Promise<Buffer>((resolve, reject) => {
      this.nativeCanvas.toBuffer(
        this.ctx.actions,
        type || 'image/png',
        encoderOptions === undefined ? 0 : encoderOptions,
        (err, val) => {
          if (err) {
            return reject(err)
          }
          resolve(val)
        })
    })
  }

  async toDataURL(type?: string, encoderOptions?: number) {
    let buffer = await this.toBuffer('image/png', encoderOptions)
    if (type === 'image/jpeg') {
      const rgba = new Buffer(UPNG.toRGBA8(UPNG.decode(buffer.buffer))[0])
      let quality = encoderOptions === undefined ? 92 : encoderOptions * 100.0
      if (typeof quality !== 'number' || isNaN(quality) || quality < 0 || quality > 100) {
        quality = 92
      }
      const jpegImageData = jpeg.encode({
        data: rgba,
        width: this.width,
        height: this.height,
      }, quality)
      buffer = jpegImageData.data
    }
    const base64 = buffer.toString('base64')
    return `data:${type};base64,${base64}`
  }
}
