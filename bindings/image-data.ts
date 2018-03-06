export class ImageData {
  readonly data: Buffer
  readonly width: number
  readonly height: number
  constructor(dataOrWidth: Buffer | number, widthOrHeight: number, height?: number) {
    if (typeof dataOrWidth === 'number') {
      this.width = dataOrWidth
      this.height = widthOrHeight
      this.data = Buffer.alloc(this.width * this.height * 4)
    } else {
      this.data = dataOrWidth
      this.width = widthOrHeight
      this.height = height!
    }
  }
}
