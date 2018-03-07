module.exports = function simple ({ ctx, canvas, type, image }) {
  ctx.lineWidth = 20
  ctx.strokeStyle = 'rgb(190,246,122)'
  ctx.moveTo(200, 200)
  ctx.lineTo(200, 400)
  ctx.moveTo(400, 400)
  ctx.bezierCurveTo(600, 600, 600, 1000, 400 , 1400)
  ctx.stroke()
  ctx.font = '"PingFang TC" 200px'
  ctx.fillStyle = '#62efff'
  ctx.fillText('ARKie 10s 做海报', 1000, 600)
  ctx.strokeStyle = '#f48fb1'
  ctx.lineWidth = 4
  ctx.strokeText('From Azure', 1600, 1400)
  ctx.drawImage(image, 0, 0, 1240, 885)
  if (type === 'skia') {
    return canvas.toBuffer().then(buffer => buffer.fill(0))
  } else {
    return new Promise((resolve, reject) => {
      canvas.toBuffer((err, data) => {
        if (err) return reject(err)
        data.fill(0)
        resolve()
      })
    })
  }
}
