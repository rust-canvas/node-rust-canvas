import * as Sharp from 'sharp'

import { Canvas } from '../index'
import { Context2D } from './context-2d'

const deasync = require('deasync')

export type CanvasCtxType = '2d' | 'webgl' | 'webgl2' | 'bitmaprenderer'

Sharp.simd(true)

export type SupportedImageType = 'jpeg' | 'png'

export class CanvasElement {

  private nativeCanvasPool: Canvas[]
  private freeCanvasPool: Canvas[]
  private canvasRefCount: number[] = Array.from({ length: this.poolSize } as ArrayLike<number>).fill(0)
  private ctx!: Context2D

  constructor(public width = 300, public height = 150, private poolSize = 1) {
    this.nativeCanvasPool = Array.from({ length: poolSize }).map(() => new Canvas(width, height))
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
    const imageType = type.split('/')[1]
    return new Promise<Buffer>((resolve, reject) => {
      nativeCanvas.toBuffer(
        this.ctx.actions,
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
      .then(raw => this.formatImage(this.ctx.imageBuffers, raw, imageType as SupportedImageType, encoderOptions))
  }

  toBufferSync(type = 'image/png', encoderOptions = 0): Buffer {
    const nativeCanvas = this.nativeCanvasPool[0]
    const raw = nativeCanvas.toBufferSync(this.ctx.actions)
    const imageType = type.split('/')[1]
    const getBuffer = (callback: (err: Error | null, data?: Buffer) => any) => {
      this.formatImage(this.ctx.imageBuffers, raw, imageType as SupportedImageType, encoderOptions)
        .then(data => callback(null, data))
        .catch(err => callback(err))
    }
    return deasync(getBuffer)()
  }

  /**
   * @param type A DOMString indicating the image format. The default format type is image/png.
   * @param encoderOptions A Number between 0 and 1 indicating image quality if the requested type is image/jpeg or image/webp.
   * If this argument is anything else, the default value for image quality is used. The default value is 0.92. Other arguments are ignored.
   */
  toDataURL(type = 'image/png', encoderOptions = 0.92) {
    if (typeof encoderOptions !== 'number' || isNaN(encoderOptions) || encoderOptions < 0 || encoderOptions > 1) {
      encoderOptions = 0.92
    }
    const buffer = this.toBufferSync(type, encoderOptions)
    const base64 = buffer.toString('base64')
    return `data:${type};base64,${base64}`
  }

  private async formatImage(imageBuffers: Buffer[], raw: Buffer, imageType: SupportedImageType, encoderOptions?: number) {
    let sharp = Sharp(raw, {
      raw: {
        width: this.width,
        height: this.height,
        channels: 4,
      }
    })
      .toFormat(
        imageType, imageType === 'jpeg' ? { quality: encoderOptions } : { compressionLevel: 9 }
      )
    if (imageBuffers.length) {
      sharp = await imageBuffers.reduce(async (acc, buffer) => {
        const instance = await acc
        const buf = await instance.overlayWith(buffer, { raw: { width: this.width, height: this.height, channels: 4 } })
          .toBuffer()
        return Sharp(buf)
      }, Promise.resolve(sharp))
    }
    return sharp.toBuffer()
  }
}
