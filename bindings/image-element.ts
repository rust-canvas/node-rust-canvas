import { ImageData } from './image-data'
import * as sizeOf from 'image-size'
import UPNG = require('upng-js')
import jpeg = require('jpeg-js')

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
      const { height, width, type } = sizeOf(this.source)
      this.height = height
      this.width = width
      if (type === 'png') {
        const rgba = new Buffer(UPNG.toRGBA8(UPNG.decode(this.source.buffer))[0])
        return new ImageData(rgba, this.width, this.height)
      }

      if (type === 'jpg') {
        const rawImageData = jpeg.decode(this.source)
        return new ImageData(rawImageData.data, this.width, this.height)
      }
    }
    return new ImageData(this.width, this.height)
  }
}
