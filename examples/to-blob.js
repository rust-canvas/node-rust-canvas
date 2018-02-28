const fs = require('fs')
const { join, resolve } = require('path')
const sharp = require("sharp")

const { CanvasElement, ImageElement } = require('../lib')

const canvas = new CanvasElement(1920 * 2, 1080 * 2)

const ctx = canvas.getContext('2d')

ctx.lineWidth = 20
ctx.strokeStyle = 'rgb(190,246,122)'
ctx.moveTo(200, 200)
ctx.lineTo(200, 400)
ctx.moveTo(400, 400)
ctx.bezierCurveTo(600, 600, 600, 1000, 400, 1400)
ctx.stroke()
ctx.font = '"PingFang TC" 200px'
ctx.fillStyle = '#62efff'
ctx.fillText('ARKie 10s 做海报', 1000, 600)
ctx.strokeStyle = '#f48fb1'
ctx.lineWidth = 4
ctx.strokeText('From Azure', 1600, 1400)

canvas.toBuffer()
  .then(buffer =>
    sharp(buffer, {
      raw: { width: 1920 * 2, height: 1080 * 2, channels: 4 },
    })
      .resize(1920, 1080)
      .png({ progressive: true, compressionLevel: 9 })
      .toFile(join(process.cwd(), 'examples', 'to-blob.png'))
  )
  .catch(e => console.error(e))
