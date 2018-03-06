import { CanvasElement } from './canvas-element'
import { CanvasGradient } from './canvas-gradient'
import { CanvasPattern } from './canvas-pattern'
import { ImageData } from './image-data'
import { ImageElement } from './image-element'
import * as Types from './interface'

export type GlobalCompositeOperationType = 'source-over' | 'source-in' | 'source-out' | 'source-atop' |
  'destination-over' | 'destination-in' | 'destination-out' | 'destination-atop' | 'lighter' | 'copy' |
  'xor' | 'multiply' | 'screen' | 'overlay' | 'darken' | 'lighten' | 'color-dodge' | 'color-burn' |
  'hard-light' | 'soft-light' | 'difference' | 'exclusion' | 'hue' | 'saturation' | 'color' | 'luminosity'

export interface Context2DState {
  fillStyle: string | CanvasGradient | CanvasPattern
  font: string
  globalAlpha: number
  globalCompositeOperation: GlobalCompositeOperationType
  lineCap: 'butt' | 'round' | 'square'
  lineDashOffset: number
  lineJoin: 'bevel' | 'round' | 'miter'
  lineWidth: number
  miterLimit: number
  shadowBlur: number
  shadowColor: string
  shadowOffsetX: number
  shadowOffsetY: number
  strokeStyle: string | CanvasGradient | CanvasPattern
  textAlign: 'left' | 'right' | 'center' | 'start' | 'end'
  textBaseline: 'top' | 'hanging' | 'middle' | 'alphabetic' | 'ideographic' | 'bottom'
  transformA: number
  transformB: number
  transformC: number
  transformD: number
  transformE: number
  transformF: number
}

export type CanvasImageSource = ImageElement | HTMLCanvasElement | HTMLVideoElement | ImageBitmap

export class Context2D {
  static defaultState: Context2DState = {
    fillStyle: '#000',
    font: '10px sans-serif',
    globalAlpha: 1.0,
    globalCompositeOperation: 'source-over',
    lineCap: 'butt',
    lineDashOffset: 0.0,
    lineJoin: 'bevel',
    lineWidth: 1.0,
    miterLimit: 10.0,
    shadowBlur: 0,
    shadowColor: '#000',
    shadowOffsetX: 0,
    shadowOffsetY: 0,
    strokeStyle: '#000',
    textAlign: 'start',
    textBaseline: 'alphabetic',
    transformA: 1,
    transformB: 0,
    transformC: 0,
    transformD: 1,
    transformE: 0,
    transformF: 0,
  }

  get currentTransform() {
    return {
      a: this.state.transformA,
      b: this.state.transformB,
      c: this.state.transformC,
      d: this.state.transformD,
      e: this.state.transformE,
      f: this.state.transformF,
    }
  }
  set currentTransform(transform: { a: number, b: number, c: number, d: number, e: number, f: number }) {
    this.state.transformA = transform.a
    this.state.transformB = transform.b
    this.state.transformC = transform.c
    this.state.transformD = transform.d
    this.state.transformE = transform.e
    this.state.transformF = transform.f
    this.actions.push({
      type: 'SET_CURRENTTRANSFORM', transform,
    })
  }
  get fillStyle(): Context2DState['fillStyle'] {
    return this.state.fillStyle
  }
  set fillStyle(fillStyle: Context2DState['fillStyle']) {
    this.state.fillStyle = fillStyle
    this.actions.push({
      type: 'SET_FILLSTYLE', fillStyle,
    })
  }
  get font(): Context2DState['font'] {
    return this.state.font
  }
  set font(font: Context2DState['font']) {
    this.state.font = font
    this.actions.push({
      type: 'SET_FONT', font,
    })
  }
  get globalAlpha(): Context2DState['globalAlpha'] {
    return this.state.globalAlpha
  }
  set globalAlpha(globalAlpha: Context2DState['globalAlpha']) {
    this.state.globalAlpha = globalAlpha
    this.actions.push({
      type: 'SET_GLOBALALPHA', globalAlpha,
    })
  }
  get globalCompositeOperation(): Context2DState['globalCompositeOperation'] {
    return this.state.globalCompositeOperation
  }
  set globalCompositeOperation(globalCompositeOperation: Context2DState['globalCompositeOperation']) {
    this.state.globalCompositeOperation = globalCompositeOperation
    this.actions.push({
      type: 'SET_GLOBALCOMPOSITEOPERATION', globalCompositeOperation,
    })
  }
  get lineCap(): Context2DState['lineCap'] {
    return this.state.lineCap
  }
  set lineCap(lineCap: Context2DState['lineCap']) {
    this.state.lineCap = lineCap
    this.actions.push({
      type: 'SET_LINECAP', lineCap,
    })
  }
  get lineDashOffset(): Context2DState['lineDashOffset'] {
    return this.state.lineDashOffset
  }
  set lineDashOffset(lineDashOffset: Context2DState['lineDashOffset']) {
    this.state.lineDashOffset = lineDashOffset
    this.actions.push({
      type: 'SET_LINEDASHOFFSET', lineDashOffset,
    })
  }
  get lineJoin(): Context2DState['lineJoin'] {
    return this.state.lineJoin
  }
  set lineJoin(lineJoin: Context2DState['lineJoin']) {
    this.state.lineJoin = lineJoin
    this.actions.push({
      type: 'SET_LINEJOIN', lineJoin,
    })
  }
  get lineWidth(): Context2DState['lineWidth'] {
    return this.state.lineWidth
  }
  set lineWidth(lineWidth: Context2DState['lineWidth']) {
    this.state.lineWidth = lineWidth
    this.actions.push({
      type: 'SET_LINEWIDTH', lineWidth,
    })
  }
  get miterLimit(): Context2DState['miterLimit'] {
    return this.state.miterLimit
  }
  set miterLimit(miterLimit: Context2DState['miterLimit']) {
    this.state.miterLimit = miterLimit
    this.actions.push({
      type: 'SET_MITERLIMIT', miterLimit,
    })
  }
  get shadowBlur(): Context2DState['shadowBlur'] {
    return this.state.shadowBlur
  }
  set shadowBlur(shadowBlur: Context2DState['shadowBlur']) {
    this.state.shadowBlur = shadowBlur
    this.actions.push({
      type: 'SET_SHADOWBLUR', shadowBlur,
    })
  }
  get shadowColor(): Context2DState['shadowColor'] {
    return this.state.shadowColor
  }
  set shadowColor(shadowColor: Context2DState['shadowColor']) {
    this.state.shadowColor = shadowColor
    this.actions.push({
      type: 'SET_SHADOWCOLOR', shadowColor,
    })
  }
  get shadowOffsetX(): Context2DState['shadowOffsetX'] {
    return this.state.shadowOffsetX
  }
  set shadowOffsetX(shadowOffsetX: Context2DState['shadowOffsetX']) {
    this.state.shadowOffsetX = shadowOffsetX
    this.actions.push({
      type: 'SET_SHADOWOFFSETX', shadowOffsetX,
    })
  }
  get shadowOffsetY(): Context2DState['shadowOffsetY'] {
    return this.state.shadowOffsetY
  }
  set shadowOffsetY(shadowOffsetY: Context2DState['shadowOffsetY']) {
    this.state.shadowOffsetY = shadowOffsetY
    this.actions.push({
      type: 'SET_SHADOWOFFSETY', shadowOffsetY,
    })
  }
  get strokeStyle(): Context2DState['strokeStyle'] {
    return this.state.strokeStyle
  }
  set strokeStyle(strokeStyle: Context2DState['strokeStyle']) {
    this.state.strokeStyle = strokeStyle
    this.actions.push({
      type: 'SET_STROKESTYLE', strokeStyle,
    })
  }
  get textAlign(): Context2DState['textAlign'] {
    return this.state.textAlign
  }
  set textAlign(textAlign: Context2DState['textAlign']) {
    this.state.textAlign = textAlign
    this.actions.push({
      type: 'SET_TEXTALIGN', textAlign,
    })
  }
  get textBaseline(): Context2DState['textBaseline'] {
    return this.state.textBaseline
  }
  set textBaseline(textBaseline: Context2DState['textBaseline']) {
    this.state.textBaseline = textBaseline
    this.actions.push({
      type: 'SET_TEXTBASELINE', textBaseline,
    })
  }
  private dashPattern?: number[]

  state: Context2DState = { ...Context2D.defaultState }

  actions: Types.Action[] = []

  constructor(public canvas: CanvasElement) {
    Object.assign(this, this.state)
  }

  arc(x: number, y: number, radius: number, startAngle: number, endAngle: number, anticlockwise = false) {
    this.actions.push({
      type: 'ARC', x, y, radius, startAngle, endAngle, anticlockwise,
    })
  }

  arcTo(x1: number, y1: number, x2: number, y2: number, radius: number) {
    this.actions.push({
      type: 'ARCTO', x1, y1, x2, y2, radius,
    })
  }

  beginPath() {
    this.actions.push({
      type: 'BEGINPATH'
    })
  }

  bezierCurveTo(cp1x: number, cp1y: number, cp2x: number, cp2y: number, x: number, y: number) {
    this.actions.push({
      type: 'BEZIERCURVETO', cp1x, cp1y, cp2x, cp2y, x, y,
    })
  }

  clearRect(x: number, y: number, width: number, height: number) {
    this.actions.push({
      type: 'CLEARRECT', x, y, width, height,
    })
  }

  clip(fillRule?: Types.FillRule) {
    this.actions.push({
      type: 'CLIP', fillRule,
    })
  }

  closePath() {
    this.actions.push({
      type: 'CLOSEPATH'
    })
  }

  createImageData(imageData: ImageData): ImageData
  createImageData(width: number, height: number): ImageData
  createImageData(widthOrImageData: number | ImageData, height?: number): ImageData {
    if (!widthOrImageData || !height) {
      throw new RangeError('NOT_SUPPORTED_ERR')
    }
    let imageData: ImageData
    if (typeof widthOrImageData === 'number') {
      imageData = new ImageData(widthOrImageData, height!)
    } else {
      imageData = new ImageData(widthOrImageData.data, widthOrImageData.width, widthOrImageData.height)
    }
    this.actions.push({
      type: 'CREATEIMAGEDATA',
      imageData,
    })
    return imageData
  }

  createLinearGradient(x0: number, y0: number, x1: number, y1: number): CanvasGradient {
    const gradient = new CanvasGradient
    this.actions.push({
      type: 'CREATELINEARGRADIENT', x0, y0, x1, y1, gradient,
    })
    return gradient
  }

  createPattern() {
    console.warn('createPattern is unsupported now')
  }

  createRadialGradient(x0: number, y0: number, r0: number, x1: number, y1: number, r1: number): CanvasGradient {
    const gradient = new CanvasGradient
    this.actions.push({
      type: 'CREATERADIALGRADIENT', x0, y0, r0, x1, y1, r1, gradient,
    })
    return gradient
  }

  drawFocusIfNeeded() {
    console.warn('drawFocusIfNeeded is unsupported now')
  }

  drawImage(
    image: CanvasImageSource,
    dxOrSx: number, dyOrSy: number, dWidthOrSWidth?: number, dHeightOrsHeight?: number,
    dx?: number, dy?: number, dWidth?: number, dHeight?: number) {
    let imageData: ImageData
    if (image instanceof ImageElement) {
      imageData = (image as ImageElement).toImageData()
    } else {
      console.warn('drawImage for non-ImageElement is unsupported now')
      return
    }
    let sx: number | undefined
    let sy: number | undefined
    let sWidth: number | undefined
    let sHeight: number | undefined
    if (typeof dx === 'number' && typeof dy === 'number' && typeof dWidth === 'number' && typeof dHeight === 'number') {
      sx = dxOrSx
      sy = dyOrSy
      sWidth = dWidthOrSWidth
      sHeight = dHeightOrsHeight
    } else {
      dx = dxOrSx
      dy = dyOrSy
      dWidth = dWidthOrSWidth
      dHeight = dHeightOrsHeight
    }
    this.actions.push({
      type: 'DRAWIMAGE',
      data: Array.from(imageData.data),
      width: imageData.width,
      height: imageData.height,
      dx,
      dy,
      dWidth: dWidth || imageData.width,
      dHeight: dHeight || imageData.height,
      sx: sx || 0,
      sy: sy || 0,
      sWidth: sWidth || dWidth || imageData.width,
      sHeight: sHeight || dHeight || imageData.height,
    })
  }

  fill(fillRule: Types.FillRule) {
    this.actions.push({
      type: 'FILL', fillRule,
    })
  }

  fillRect(x: number, y: number, width: number, height: number) {
    this.actions.push({
      type: 'FILLRECT', x, y, width, height,
    })
  }

  fillText(text: string, x: number, y: number, maxWidth?: number) {
    this.actions.push({
      type: 'FILLTEXT', text, x, y, maxWidth,
    })
  }

  getImageData(sx: number, sy: number, sw: number, sh: number) {
    // to implement getImageData after binding rust-canvas
    console.info(sx, sy, sw, sh)
  }

  getLineDash() {
    return this.dashPattern
  }

  isPointInPath(x: number, y: number, fillRule?: Types.FillRule, path?: Path2D) {
    // to implement isPointInPath after binding rust-canvas
    console.info(x, y, fillRule, path)
  }

  isPointInStroke(x: number, y: number) {
    // to implement isPointInStroke after binding rust-canvas
    console.info(x, y)
  }

  lineTo(x: number, y: number) {
    this.actions.push({
      type: 'LINETO', x, y,
    })
  }

  measureText(text: string) {
    this.actions.push({
      type: 'MEASURETEXT', text,
    })
  }

  moveTo(x: number, y: number) {
    this.actions.push({
      type: 'MOVETO', x, y,
    })
  }

  putImageData(imageData: ImageData, dx: number, dy: number, dirtyX?: number, dirtyY?: number, dirtyWidth?: number, dirtyHeight?: number) {
    this.actions.push({
      type: 'PUTIMAGEDATA', imageData, dx, dy, dirtyX, dirtyY, dirtyWidth, dirtyHeight,
    })
  }

  quadraticCurveTo(cpx: number, cpy: number, x: number, y: number) {
    this.actions.push({
      type: 'QUADRATICCURVETO', cpx, cpy, x, y,
    })
  }

  rect(x: number, y: number, width: number, height: number) {
    this.actions.push({
      type: 'RECT', x, y, width, height,
    })
  }

  restore() {
    this.actions.push({
      type: 'RESTORE', state: this.state,
    })
  }

  rotate(angle: number) {
    const sin = Math.sin(angle)
    const cos = Math.cos(angle)
    this.transform(cos, sin, -sin, cos, 0, 0)
  }

  save() {
    this.actions.push({
      type: 'SAVE', state: this.state,
    })
  }

  scale(x: number, y: number) {
    this.transform(x, 0, 0, y, 0, 0)
  }

  setLineDash(segments: number[]) {
    const { length } = segments
    if (length % 2 === 0) {
      this.dashPattern = segments
    } else {
      this.dashPattern = [...segments, ...segments]
    }
    this.actions.push({
      type: 'SETLINEDASH', segments,
    })
  }

  setTransform(a: number, b: number, c: number, d: number, e: number, f: number) {
    this.state.transformA = a
    this.state.transformB = b
    this.state.transformC = c
    this.state.transformD = d
    this.state.transformE = e
    this.state.transformF = f
    this.actions.push({
      type: 'SETTRANSFORM', a, b, c, d, e, f,
    })
  }

  stroke() {
    this.actions.push({
      type: 'STROKE',
    })
  }

  strokeRect(x: number, y: number, width: number, height: number) {
    this.actions.push({
      type: 'STROKERECT', x, y, width, height,
    })
  }

  strokeText(text: string, x: number, y: number, maxWidth?: number) {
    this.actions.push({
      type: 'STROKETEXT', text, x, y, maxWidth,
    })
  }

  transform(a: number, b: number, c: number, d: number, e: number, f: number) {
    a *= this.state.transformA
    b += this.state.transformB
    c += this.state.transformC
    d *= this.state.transformD
    e += this.state.transformE
    f += this.state.transformF
    this.actions.push({
      type: 'SETTRANSFORM', a, b, c, d, e, f,
    })
  }

  translate(x: number, y: number) {
    this.transform(1, 0, 0, 1, x, y)
  }
}
