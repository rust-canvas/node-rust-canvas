const fs = require('fs')
const { join } = require('path')
const { CanvasElement, ImageElement } = require('../lib')

const canvas = new CanvasElement(1920 * 2, 1080 * 2)
const image = new ImageElement(2481, 1773)
image.src = fs.readFileSync(join(process.cwd(), 'examples', 'fixtures', 'image.png'))

module.exports = {
  canvas,
  ctx: canvas.getContext('2d'),
  image,
  type: 'skia',
}
