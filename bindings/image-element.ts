import { ImageData } from './image-data'
import axios from 'axios'
import * as fs from 'fs'
import * as util from 'util'
import * as sizeOf from 'image-size'

const readFileAsync = util.promisify(fs.readFile)

export class ImageElement {
  src!: string | Buffer
  constructor(public width: number, public height: number) { }

  async toImageData() {
    if (this.src) {
      if (typeof this.src === 'string') {
        if (this.src.startsWith('http')) {
          const response = await axios({
            method: 'get',
            url: this.src,
            responseType: 'arraybuffer'
          })
          const data = Uint8ClampedArray.from(response.data)
          const { height, width } = sizeOf(response.data)
          this.height = height
          this.width = width
          return new ImageData(data, this.width, this.height)
        } else {
          const buffer = await readFileAsync(this.src)
          const data = Uint8ClampedArray.from(buffer)
          const { height, width } = sizeOf(buffer)
          this.height = height
          this.width = width
          return new ImageData(data, this.width, this.height)
        }
      } else if (Buffer.isBuffer(this.src)) {
        const data = Uint8ClampedArray.from(this.src)
        const { height, width } = sizeOf(this.src)
          this.height = height
          this.width = width
        return new ImageData(data, this.width, this.height)
      }
    }
    return new ImageData(this.width, this.height)
  }
}
