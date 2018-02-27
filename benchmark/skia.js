const { Canvas } = require('../native')
const { CanvasElement } = require('../lib/canvas-element')

const canvas = new CanvasElement(1920 * 2, 1080 * 2)

module.exports = {
  canvas,
  ctx: canvas.getContext('2d'),
}
