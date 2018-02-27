import { ImageData } from './image-data'
import { Context2DState } from './context-2d'

export type FillRule = 'nonzero' | 'evenodd'

export type Action = ArcAction | ArcToAction | BeginPathAction | BezierCurveToAction | ClearRectAction |
  ClipAction | ClosePathAction | CreateLinearGradientAction | CreateImageDataAction | CreateRadialGradientAction |
  FillAction | FillRectAction | FillTextAction | LineToAction | MeasureTextAction |
  MoveToAction | PutImageDataAction | QuadraticCurveToAction | RectAction | RestoreAction |
  RotateAction | SaveAction | ScaleAction | SetLineDashAction | SetTransformAction |
  StrokeAction | StrokeRectAction | StrokeTextAction | TransformAction | TranslateAction |
  SetCurrentTransformAction | SetFillStyleAction | SetFontAction | SetGlobalAlphaAction | SetGlobalCompositeOperationAction |
  SetLineCapAction | SetLineDashOffsetAction | SetLineJoinAction | SetLineWidthAction | SetMiterLimitAction |
  SetShadowBlurAction | SetShadowColorAction | SetShadowOffsetXAction | SetShadowOffsetYAction | SetStrokeStyleAction |
  SetTextAlignAction | SetTextBaselineAction

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

export interface FillTextAction {
  type: 'FILLTEXT'
  text: string
  x: number
  y: number
  maxWidth?: number
}

export interface LineToAction {
  type: 'LINETO'
  x: number
  y: number
}

export interface MeasureTextAction {
  type: 'MEASURETEXT'
  text: string
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

export interface RestoreAction {
  type: 'RESTORE'
  state: Context2DState
}

export interface RotateAction {
  type: 'ROTATE'
  angle: number
}

export interface SaveAction {
  type: 'SAVE'
  state: Context2DState
}

export interface ScaleAction {
  type: 'SCALE'
  x: number
  y: number
}

export interface SetLineDashAction {
  type: 'SETLINEDASH'
  segments: number[]
}

export interface SetTransformAction {
  type: 'SETTRANSFORM'
  a: number
  b: number
  c: number
  d: number
  e: number
  f: number
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

export interface StrokeTextAction {
  type: 'STROKETEXT'
  text: string
  x: number
  y: number
  maxWidth?: number
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

export interface SetCurrentTransformAction {
  type: 'SET_CURRENTTRANSFORM'
  transform: { a: number, b: number, c: number, d: number, e: number, f: number }
}

export interface SetFillStyleAction {
  type: 'SET_FILLSTYLE'
  fillStyle: Context2DState['fillStyle']
}

export interface SetFontAction {
  type: 'SET_FONT'
  font: Context2DState['font']
}

export interface SetGlobalAlphaAction {
  type: 'SET_GLOBALALPHA'
  globalAlpha: number
}

export interface SetGlobalCompositeOperationAction {
  type: 'SET_GLOBALCOMPOSITEOPERATION'
  globalCompositeOperation: Context2DState['globalCompositeOperation']
}

export interface SetLineCapAction {
  type: 'SET_LINECAP'
  lineCap: Context2DState['lineCap']
}

export interface SetLineDashOffsetAction {
  type: 'SET_LINEDASHOFFSET'
  lineDashOffset: Context2DState['lineDashOffset']
}

export interface SetLineJoinAction {
  type: 'SET_LINEJOIN'
  lineJoin: Context2DState['lineJoin']
}

export interface SetLineWidthAction {
  type: 'SET_LINEWIDTH'
  lineWidth: Context2DState['lineWidth']
}

export interface SetMiterLimitAction {
  type: 'SET_MITERLIMIT'
  miterLimit: Context2DState['miterLimit']
}

export interface SetShadowBlurAction {
  type: 'SET_SHADOWBLUR'
  shadowBlur: Context2DState['shadowBlur']
}

export interface SetShadowColorAction {
  type: 'SET_SHADOWCOLOR'
  shadowColor: Context2DState['shadowColor']
}

export interface SetShadowOffsetXAction {
  type: 'SET_SHADOWOFFSETX'
  shadowOffsetX: Context2DState['shadowOffsetX']
}

export interface SetShadowOffsetYAction {
  type: 'SET_SHADOWOFFSETY'
  shadowOffsetY: Context2DState['shadowOffsetY']
}

export interface SetStrokeStyleAction {
  type: 'SET_STROKESTYLE'
  strokeStyle: Context2DState['strokeStyle']
}

export interface SetTextAlignAction {
  type: 'SET_TEXTALIGN'
  textAlign: Context2DState['textAlign']
}

export interface SetTextBaselineAction {
  type: 'SET_TEXTBASELINE'
  textBaseline: Context2DState['textBaseline']
}
