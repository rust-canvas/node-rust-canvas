import { Action } from '../src/interface'
import { Context2DState } from '../src/context-2d'

export class Canvas {
  constructor(width: number, height: number)

  toBuffer(actions: Array<Action>): Buffer
}
