import { ImageData } from './image-data'

export type Action = ArcAction | ArcToAction | BeginPathAction | BezierCurveToAction | ClearRectAction |
  ClipAction | ClosePathAction | CreateLinearGradientAction | CreateImageDataAction

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
}

export interface CreateImageDataAction {
  type: 'CREATEIMAGEDATA'
  imageData: ImageData
}
