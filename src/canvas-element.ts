export type CanvasCtxType = '2d' | 'webgl' | 'webgl2' | 'bitmaprenderer'

export class CanvasElement {
  private _width = 300
  private _height = 150

  set width (width: number) {
    this._width = width
  }

  get width() {
    return this._width
  }

  set height(height: number) {
    this._height = height
  }

  get height() {
    return this._height
  }

  getContext(ctxType: CanvasCtxType) {
    switch (ctxType) {
      case '2d':
        return
      default:
        console.warn('not implement')
    }
  }
}
