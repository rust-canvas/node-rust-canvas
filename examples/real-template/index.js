const fs = require('fs')
const { join, resolve } = require('path')

const { CanvasElement, ImageElement } = require('../../lib')

const canvas = new CanvasElement(1080, 1920)

const ctx = canvas.getContext('2d')

function drawImage(name, ...params) {
  const image = new ImageElement()
  image.src = fs.readFileSync(join(process.cwd(), 'examples', 'real-template', name))
  ctx.drawImage(image, ...params)
}

drawImage('6423a9e3-665c-4b4a-aaa4-5b9478c2f150.png', -540, -960, 1080, 1920)
drawImage('257bf48a-bf98-4e98-bfe5-410d71ec80b3.png', -505.5, -412.5, 1011, 825)
// drawImage('63611baa-2888-46f3-a2dd-bfb54d7f4482.png', -562.5, -1218, 1125, 2436)
// drawImage('a68be70f-df59-494b-8e44-5e7a176afc31.png', -540, -960, 1080, 1920)
// drawImage('ff2aaa4a-4996-4230-a3a3-761903878464.png', -540, -960, 1080, 1920)
// drawImage('7f307cfe-1aa4-41ce-884b-762892f8bf18.png', -48, -68, 96, 136)
// drawImage('1517989108553_e6b7e74f-3728-4514-ac88-29fa31a10e9b.png', -135, -135, 270, 270)

const base64 = canvas.toDataURL()
fs.writeFileSync(join(process.cwd(), 'examples', 'real-template', 'result.png'), base64.split(',')[1], {
  encoding: 'base64'
})
