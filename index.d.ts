import { Action } from './bindings/interface'
import { Context2DState } from './bindings/context-2d'

export class Canvas {
  constructor(width: number, height: number)

  toBuffer(actions: Array<Action>, cb: (err: Error, value: Buffer) => any): Buffer
}
