import { Action } from '../src/interface'
import { Context2DState } from '../src/context-2d'

export class Canvas {
  constructor(width: number, height: number)

  toBlob(actions: Array<Action>, states: Array<Context2DState>): Buffer
}
