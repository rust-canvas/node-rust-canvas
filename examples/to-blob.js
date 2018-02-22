const { Canvas } = require('../native')

const canvas = new Canvas(1920, 1080)

const buffer = canvas.toBlob()

console.log(buffer)
