import { ImageData } from './image-data'
import * as sizeOf from 'image-size'
import * as Sharp from 'sharp'

const deasync = require('deasync')

export class ImageElement {
  source!: Buffer
  constructor(public width: number, public height: number) { }

  get src() {
    return this.source
  }
  set src(val: string | Buffer) {
    if ('string' === typeof val && 0 === val.indexOf('data:')) {
      this.source = new Buffer(val.slice(val.indexOf(',') + 1), 'base64')
    } else if (Buffer.isBuffer(val)) {
      this.source = val
    }
  }

  toImageData() {
    if (this.source) {
      const { height, width } = sizeOf(this.source)
      this.height = height
      this.width = width
      let sharp = Sharp(this.source).raw()
      const getMeta = deasync((callback: any) => sharp.metadata().then(m => callback(null, m)).catch(e => callback(e)))
      const meta = getMeta()
      if (!meta.hasAlpha) {
        const alpha = Buffer.alloc(width * height).fill(255)
        sharp = sharp.joinChannel(alpha, {
          raw: {
            width: width,
            height: height,
            channels: 1
          }
        })
      }
      const syncToBuffer = deasync(sharp.toBuffer.bind(sharp))
      const raw = syncToBuffer()
      return new ImageData(raw, this.width, this.height)
    }
    return new ImageData(this.width, this.height)
  }
}
