export class CanvasGradient {

  offset = 0
  color = '#000'

  /**
   * @param offset A number between 0 and 1. An INDEX_SIZE_ERR is raised, if the number is not in that range.
   * @param color A CSS <color>. A SYNTAX_ERR is raised, if the value can not be parsed as a CSS <color> value.
   */
  addColorStop(offset: number, color: string) {
    this.offset = offset
    this.color = color
  }
}
