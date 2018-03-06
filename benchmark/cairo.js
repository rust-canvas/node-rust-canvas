const fs = require('fs')
const { join } = require('path')
const Canvas = require('canvas')

const canvas = new Canvas(1920 * 2, 1080 * 2)
const image = new Canvas.Image(2481, 1773)
image.src = fs.readFileSync(join(process.cwd(), 'examples', 'fixtures', 'image.png'))

module.exports = {
  canvas,
  ctx: canvas.getContext('2d'),
  image,
  type: 'cairo'
}
