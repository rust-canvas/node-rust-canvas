import * as os from 'os'
import * as jpeg from 'jpeg-js'
import * as UPNG from 'upng-js'

import { Canvas } from '../index'
import { Context2D } from './context-2d'

export type CanvasCtxType = '2d' | 'webgl' | 'webgl2' | 'bitmaprenderer'

const counterSymbol = Symbol('counter')
let i = 0

export class CanvasElement {

  private nativeCanvasPool: Canvas[]
  private freeCanvasPool: Canvas[]
  private canvasRefCount: number[] = Array.from({ length: this.poolSize } as ArrayLike<number>).fill(0)
  private ctx!: Context2D

  constructor(public width = 300, public height = 150, private poolSize = os.cpus().length) {
    this.nativeCanvasPool = Array.from({ length: poolSize }).map(() => {
      const canvas = new Canvas(width, height)
      canvas[counterSymbol] = i++
      return canvas
    })
    this.freeCanvasPool = [...this.nativeCanvasPool]
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

  toBuffer(type = 'image/png', encoderOptions = 0) {
    const { canvasRefCount } = this
    const nativeCanvas = this.freeCanvasPool.length
      ? this.freeCanvasPool.pop()!
      : this.nativeCanvasPool[canvasRefCount.indexOf([...canvasRefCount].sort((a, b) => a - b)[0])]

    const canvasIndex = this.nativeCanvasPool.indexOf(nativeCanvas)
    canvasRefCount[canvasIndex]++
    return new Promise<Buffer>((resolve, reject) => {
      nativeCanvas.toBuffer(
        this.ctx.actions,
        type,
        encoderOptions,
        (err, val) => {
          if (!(--canvasRefCount[canvasIndex])) {
            this.freeCanvasPool.push(nativeCanvas)
          }
          if (err) {
            return reject(err)
          }
          resolve(val)
        })
    })
  }

  toBufferSync(type = 'image/png', encoderOptions = 0) {
    const nativeCanvas = this.nativeCanvasPool[0]
    return nativeCanvas.toBufferSync(this.ctx.actions, type, encoderOptions)
  }

  toDataURL(type?: string, encoderOptions = 92) {
    let buffer = this.toBufferSync('image/png', encoderOptions)
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
