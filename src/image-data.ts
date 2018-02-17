export class ImageData {
  constructor(public width: number, public height: number) { }

  clone() {
    return new ImageData(this.width, this.height)
  }
}
