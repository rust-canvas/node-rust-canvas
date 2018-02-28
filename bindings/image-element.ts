import { ImageData } from './image-data'
import axios from 'axios'
import * as fs from 'fs'
import * as util from 'util'

const readFileAsync = util.promisify(fs.readFile)

export class ImageElement {
  readonly width: number
  readonly height: number
  src!: string
  constructor(width: number, height: number) {
    this.width = width
    this.height = height
  }

  async toImageData() {
    if (this.src) {
      if (this.src.startsWith('http')) {
        const response = await axios({
          method: 'get',
          url: this.src,
          responseType: 'arraybuffer'
        })
        const data = Uint8ClampedArray.from(response.data)
        return new ImageData(data, this.width, this.height)
      } else {
        const buffer = await readFileAsync(this.src)
        const data = Uint8ClampedArray.from(buffer)
        return new ImageData(data, this.width, this.height)
      }
    } else {
      return new ImageData(this.width, this.height)
    }
  }
}
