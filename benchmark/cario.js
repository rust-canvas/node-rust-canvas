const Canvas = require('canvas')

const canvas = new Canvas(1920 * 2, 1080 * 2)

const toBuffer = canvas.toBuffer
canvas.toBuffer = function() {
  return new Promise((resolve, reject) => {
    toBuffer.call(this, (err, result) => {
      if (err) return reject(err)
      resolve(result)
    })
  })
}

module.exports = {
  canvas,
  ctx: canvas.getContext('2d')
}
