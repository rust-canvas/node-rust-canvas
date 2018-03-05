const Canvas = require('canvas')

const canvas = new Canvas(1920 * 2, 1080 * 2)

module.exports = {
  canvas,
  ctx: canvas.getContext('2d'),
  type: 'cairo'
}
