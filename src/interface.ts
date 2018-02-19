import { ImageData } from './image-data'

export type FillRule = 'nonzero' | 'evenodd'

export type Action = ArcAction | ArcToAction | BeginPathAction | BezierCurveToAction | ClearRectAction |
  ClipAction | ClosePathAction | CreateLinearGradientAction | CreateImageDataAction | CreateRadialGradientAction |
  FillAction | FillRectAction | LineToAction | MoveToAction | PutImageDataAction | QuadraticCurveToAction |
  RectAction | RotateAction | ScaleAction | StrokeAction | StrokeRectAction | TransformAction | TranslateAction

export interface ArcAction {
  type: 'ARC'
  x: number
  y: number
  radius: number
  startAngle: number
  endAngle: number
  anticlockwise: boolean
}

export interface ArcToAction {
  type: 'ARCTO'
  x1: number
  y1: number
  x2: number
  y2: number
  radius: number
}

export interface BeginPathAction { type: 'BEGINPATH' }

export interface BezierCurveToAction {
  type: 'BEZIERCURVETO', cp1x: number, cp1y: number, cp2x: number, cp2y: number, x: number, y: number,
}

export interface ClearRectAction {
  type: 'CLEARRECT'
  x: number
  y: number
  width: number
  height: number
}

export interface ClipAction {
  type: 'CLIP'
  fillRule?: 'nonzero' | 'evenodd'
}

export interface ClosePathAction { type: 'CLOSEPATH' }

export interface CreateLinearGradientAction {
  type: 'CREATELINEARGRADIENT'
  x0: number
  y0: number
  x1: number
  y1: number
  gradient: CanvasGradient
}

export interface CreateImageDataAction {
  type: 'CREATEIMAGEDATA'
  imageData: ImageData
}

export interface CreateRadialGradientAction {
  type: 'CREATERADIALGRADIENT'
  x0: number
  y0: number
  r0: number
  x1: number
  y1: number
  r1: number
  gradient: CanvasGradient
}

export interface FillAction {
  type: 'FILL'
  fillRule?: FillRule
}

export interface FillRectAction {
  type: 'FILLRECT'
  x: number
  y: number
  width: number
  height: number
}

export interface LineToAction {
  type: 'LINETO'
  x: number
  y: number
}

export interface MoveToAction {
  type: 'MOVETO'
  x: number
  y: number
}

export interface PutImageDataAction {
  type: 'PUTIMAGEDATA'
  dx: number
  dy: number
  dirtyX?: number
  dirtyY?: number
  dirtyWidth?: number
  dirtyHeight?: number
}

export interface QuadraticCurveToAction {
  type: 'QUADRATICCURVETO'
  cpx: number
  cpy: number
  x: number
  y: number
}

export interface RectAction {
  type: 'RECT'
  x: number
  y: number
  width: number
  height: number
}

export interface RotateAction {
  type: 'ROTATE'
  angle: number
}

export interface ScaleAction {
  type: 'SCALE'
  x: number
  y: number
}

export interface StrokeAction {
  type: 'STROKE'
}

export interface StrokeRectAction {
  type: 'STROKERECT'
  x: number
  y: number
  width: number
  height: number
}

export interface TransformAction {
  type: 'TRANSFORM'
  a: number
  b: number
  c: number
  d: number
  e: number
  f: number
}

export interface TranslateAction {
  type: 'TRANSLATE'
  x: number
  y: number
}
