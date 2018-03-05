import { ImageData } from './image-data'
import axios from 'axios'
import * as fs from 'fs'
import * as util from 'util'
import * as sizeOf from 'image-size'
import UPNG = require('upng-js')
import jpeg = require('jpeg-js')

const readFileAsync = util.promisify(fs.readFile)

export class ImageElement {
  src!: string | Buffer
  constructor(public width: number, public height: number) { }

  async toImageData() {
    if (this.src) {
      let buffer: Buffer
      if (typeof this.src === 'string') {
        if (this.src.startsWith('http')) {
          const response = await axios({
            method: 'get',
            url: this.src,
            responseType: 'arraybuffer'
          })
          buffer = response.data
        } else {
          buffer = await readFileAsync(this.src)
        }
      } else if (Buffer.isBuffer(this.src)) {
        buffer = this.src
      } else {
        return new ImageData(this.width, this.height)
      }
      const { height, width, type } = sizeOf(buffer)
      this.height = height
      this.width = width
      let data: Uint8ClampedArray
      if (type === 'png') {
        const rgba = new Buffer(UPNG.toRGBA8(UPNG.decode(buffer.buffer))[0])
        data = Uint8ClampedArray.from(rgba)
      } else if (type === 'jpg') {
        const rawImageData = jpeg.decode(buffer)
        data = Uint8ClampedArray.from(rawImageData.data)
      } else {
        return new ImageData(this.width, this.height)
      }
      return new ImageData(data, this.width, this.height)
    }
    return new ImageData(this.width, this.height)
  }
}
