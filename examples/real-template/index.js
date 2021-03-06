const isInBrowser = typeof window !== 'undefined'

const canvas = isInBrowser
  ? document.getElementById("canvas")
  : new (require('../../lib').CanvasElement)(1080, 1920)

const ctx = canvas.getContext('2d')

function drawImage(name, ...params) {
  const image = isInBrowser ? new Image() : new (require('../../lib').ImageElement)()
  image.src = isInBrowser
    ? './' + name
    : require('fs').readFileSync(require('path').join(process.cwd(), 'examples', 'real-template', name))
  ctx.drawImage(image, ...params)
}

ctx.clearRect(0, 0, 1080, 1920)
ctx.fillStyle = '#fff'
ctx.fillRect(0, 0, 1080, 1920)
ctx.save()
  ctx.save()
  ctx.transform(1, 0, 0, 1, 540, 960)
    ctx.save()
    ctx.beginPath()
    ctx.rect(-540, -960, 1080, 1920)
    ctx.clip(undefined)
      ctx.save()
      ctx.beginPath()
      ctx.moveTo(-540, -960)
      ctx.lineTo(540, -960)
      ctx.lineTo(540, 960)
      ctx.lineTo(-540, 960)
      ctx.lineTo(-540, -960)
      ctx.closePath()
      ctx.restore()
    ctx.restore()
  ctx.restore()
  ctx.save()
  ctx.transform(1, 0, 0, 1, 540, 960)
    ctx.save()
    ctx.fillStyle = 'rgb(0,0,0)'
    drawImage('6423a9e3-665c-4b4a-aaa4-5b9478c2f150.png', -540, -960, 1080, 1920)
    ctx.restore()
  ctx.restore()
  ctx.save()
  ctx.transform(1, 0, 0, 1, 540, 443)
    ctx.save()
    ctx.transform(0.9963636363636363, 0, 0, 0.9963636363636363, 0, 0)
    ctx.fillStyle = 'rgb(0,0,0)'
    drawImage('257bf48a-bf98-4e98-bfe5-410d71ec80b3.png', -505.5, -412.5, 1011, 825)
    ctx.restore()
  ctx.restore()
  ctx.save()
  ctx.transform(1, 0, 0, 1, 540.0026386163597, 442.5911661331239)
    ctx.save()
    ctx.transform(1.0951999203278557, 0, 0, 1.0951999203278557, 0, 0)
    ctx.fillStyle = 'rgb(0,0,0)'
    // drawImage('63611baa-2888-46f3-a2dd-bfb54d7f4482.png', -562.5, -1218, 1125, 2436)
    ctx.restore()
  ctx.restore()
  ctx.save()
  ctx.transform(1, 0, 0, 1, 540, 960)
    ctx.save()
    ctx.fillStyle = 'rgb(0,0,0)'
    // drawImage('a68be70f-df59-494b-8e44-5e7a176afc31.png', -540, -960, 1080, 1920)
    ctx.restore()
  ctx.restore()
  ctx.save()
  ctx.transform(1, 0, 0, 1, 540, 960)
    ctx.save()
    ctx.fillStyle = 'rgb(0,0,0)'
    // drawImage('ff2aaa4a-4996-4230-a3a3-761903878464.png', -540, -960, 1080, 1920)
    ctx.restore()
  ctx.restore()
  ctx.save()
  ctx.transform(1, 0, 0, 1, 593.6415047889432, 1380.388820252522)
    ctx.save()
    ctx.transform(1, 0, 0, 1, 5.000000129484761e-7, -35.11767800000001)
      ctx.save()
      ctx.transform(1, 0, 0, 1, 0, -286.65)
      ctx.fillStyle = '#000000'
      ctx.font = 'normal  195px "PingFang TC"'
      ctx.fillText('雷', -97.5, 61.42499999999998, undefined)
      ctx.restore()
      ctx.save()
      ctx.transform(1, 0, 0, 1, 0, -95.54999999999995)
      ctx.fillStyle = '#000000'
      ctx.font = 'normal  195px "PingFang TC"'
      ctx.fillText('厉', -97.5, 61.42499999999998, undefined)
      ctx.restore()
      ctx.save()
      ctx.transform(1, 0, 0, 1, 0, 95.55000000000001)
      ctx.fillStyle = '#000000'
      ctx.font = 'normal  195px "PingFang TC"'
      ctx.fillText('风', -97.5, 61.42499999999998, undefined)
      ctx.restore()
      ctx.save()
      ctx.transform(1, 0, 0, 1, 0, 286.65)
      ctx.fillStyle = '#000000'
      ctx.font = 'normal  195px "PingFang TC"'
      ctx.fillText('行', -97.5, 61.42499999999998, undefined)
      ctx.restore()
    ctx.restore()
  ctx.restore()
  ctx.save()
  ctx.transform(1, 0, 0, 1, 739.8764247838543, 1398.2789736521404)
    ctx.save()
    ctx.transform(1, 0, 0, 1, 0, -52.30787299999997)
      ctx.save()
      ctx.transform(1, 0, 0, 1, 0, -349.86)
      ctx.fillStyle = '#000000'
      ctx.font = 'normal  51px "PingFang TC"'
      ctx.fillText('有', -25.5, 16.064999999999998, undefined)
      ctx.restore()
      ctx.save()
      ctx.transform(1, 0, 0, 1, 0, -299.88)
      ctx.fillStyle = '#000000'
      ctx.font = 'normal  51px "PingFang TC"'
      ctx.fillText('执', -25.5, 16.064999999999998, undefined)
      ctx.restore()
      ctx.save()
      ctx.transform(1, 0, 0, 1, 0, -249.90000000000003)
      ctx.fillStyle = '#000000'
      ctx.font = 'normal  51px "PingFang TC"'
      ctx.fillText('着', -25.5, 16.064999999999998, undefined)
      ctx.restore()
      ctx.save()
      ctx.transform(1, 0, 0, 1, 0, -199.92000000000002)
      ctx.fillStyle = '#000000'
      ctx.font = 'normal  51px "PingFang TC"'
      ctx.fillText('的', -25.5, 16.064999999999998, undefined)
      ctx.restore()
      ctx.save()
      ctx.transform(1, 0, 0, 1, 0, -149.94000000000003)
      ctx.fillStyle = '#000000'
      ctx.font = 'normal  51px "PingFang TC"'
      ctx.fillText('梦', -25.5, 16.064999999999998, undefined)
      ctx.restore()
      ctx.save()
      ctx.transform(1, 0, 0, 1, 0, -99.96000000000004)
      ctx.fillStyle = '#000000'
      ctx.font = 'normal  51px "PingFang TC"'
      ctx.fillText('也', -25.5, 16.064999999999998, undefined)
      ctx.restore()
      ctx.save()
      ctx.transform(1, 0, 0, 1, 0, -49.98000000000002)
      ctx.fillStyle = '#000000'
      ctx.font = 'normal  51px "PingFang TC"'
      ctx.fillText('有', -25.5, 16.064999999999998, undefined)
      ctx.restore()
      ctx.save()
      ctx.fillStyle = '#000000'
      ctx.font = 'normal  51px "PingFang TC"'
      ctx.fillText('洒', -25.5, 16.064999999999998, undefined)
      ctx.restore()
      ctx.save()
      ctx.transform(1, 0, 0, 1, 0, 49.97999999999996)
      ctx.fillStyle = '#000000'
      ctx.font = 'normal  51px "PingFang TC"'
      ctx.fillText('脱', -25.5, 16.064999999999998, undefined)
      ctx.restore()
      ctx.save()
      ctx.transform(1, 0, 0, 1, 0, 99.95999999999998)
      ctx.fillStyle = '#000000'
      ctx.font = 'normal  51px "PingFang TC"'
      ctx.fillText('的', -25.5, 16.064999999999998, undefined)
      ctx.restore()
      ctx.save()
      ctx.transform(1, 0, 0, 1, 0, 149.93999999999994)
      ctx.fillStyle = '#000000'
      ctx.font = 'normal  51px "PingFang TC"'
      ctx.fillText('生', -25.5, 16.064999999999998, undefined)
      ctx.restore()
      ctx.save()
      ctx.transform(1, 0, 0, 1, 0, 199.91999999999996)
      ctx.fillStyle = '#000000'
      ctx.font = 'normal  51px "PingFang TC"'
      ctx.fillText('活', -25.5, 16.064999999999998, undefined)
      ctx.restore()
      ctx.save()
      ctx.transform(1, 0, 0, 1, 0, 249.89999999999998)
      ctx.fillStyle = '#000000'
      ctx.font = 'normal  51px "PingFang TC"'
      ctx.fillText('和', -25.5, 16.064999999999998, undefined)
      ctx.restore()
      ctx.save()
      ctx.transform(1, 0, 0, 1, 0, 299.88)
      ctx.fillStyle = '#000000'
      ctx.font = 'normal  51px "PingFang TC"'
      ctx.fillText('红', -25.5, 16.064999999999998, undefined)
      ctx.restore()
      ctx.save()
      ctx.transform(1, 0, 0, 1, 0, 349.86)
      ctx.fillStyle = '#000000'
      ctx.font = 'normal  51px "PingFang TC"'
      ctx.fillText('包', -25.5, 16.064999999999998, undefined)
      ctx.restore()
    ctx.restore()
  ctx.restore()
  ctx.save()
  ctx.transform(1, 0, 0, 1, 834.672085579658, 1211.6347144913552)
    ctx.save()
    ctx.transform(1, 0, 0, 1, 0, -66.640985)
      ctx.save()
      ctx.transform(1, 0, 0, 1, 0, -132.3)
      ctx.fillStyle = '#000000'
      ctx.font = 'normal  90px "PingFang TC"'
      ctx.fillText('谨', -45, 28.349999999999994, undefined)
      ctx.restore()
      ctx.save()
      ctx.transform(1, 0, 0, 1, 0, -44.10000000000002)
      ctx.fillStyle = '#000000'
      ctx.font = 'normal  90px "PingFang TC"'
      ctx.fillText('贺', -45, 28.349999999999994, undefined)
      ctx.restore()
      ctx.save()
      ctx.transform(1, 0, 0, 1, 0, 44.099999999999994)
      ctx.fillStyle = '#000000'
      ctx.font = 'normal  90px "PingFang TC"'
      ctx.fillText('新', -45, 28.349999999999994, undefined)
      ctx.restore()
      ctx.save()
      ctx.transform(1, 0, 0, 1, 0, 132.3)
      ctx.fillStyle = '#000000'
      ctx.font = 'normal  90px "PingFang TC"'
      ctx.fillText('春', -45, 28.349999999999994, undefined)
      ctx.restore()
    ctx.restore()
  ctx.restore()
  ctx.save()
  ctx.transform(1, 0, 0, 1, 939.0283286794227, 1213.0934002049873)
    ctx.save()
    ctx.transform(1, 0, 0, 1, 4.999999987376214e-7, -109.28229950000002)
      ctx.save()
      ctx.transform(1, 0, 0, 1, 0, -88.20000000000002)
      ctx.fillStyle = '#000000'
      ctx.font = 'normal  90px "PingFang TC"'
      ctx.fillText('雷', -45, 28.349999999999994, undefined)
      ctx.restore()
      ctx.save()
      ctx.transform(1, 0, 0, 1, 0, -2.842170943040401e-14)
      ctx.fillStyle = '#000000'
      ctx.font = 'normal  90px "PingFang TC"'
      ctx.fillText('佳', -45, 28.349999999999994, undefined)
      ctx.restore()
      ctx.save()
      ctx.transform(1, 0, 0, 1, 0, 88.19999999999999)
      ctx.fillStyle = '#000000'
      ctx.font = 'normal  90px "PingFang TC"'
      ctx.fillText('音', -45, 28.349999999999994, undefined)
      ctx.restore()
    ctx.restore()
  ctx.restore()
  ctx.save()
  ctx.transform(1, 0, 0, 1, 964.3750419449117, 1791.263570829159)
    ctx.save()
    ctx.transform(1.1838235294117647, 0, 0, 1.1838235294117647, 0, 0)
    ctx.fillStyle = 'rgb(0,0,0)'
    // drawImage('7f307cfe-1aa4-41ce-884b-762892f8bf18.png', -48, -68, 96, 136)
    ctx.restore()
  ctx.restore()
  ctx.save()
  ctx.transform(1, 0, 0, 1, 165.3658536585367, 1754.8997233368143)
    ctx.save()
    ctx.fillStyle = 'rgb(0,0,0)'
    // drawImage('1517989108553_e6b7e74f-3728-4514-ac88-29fa31a10e9b.png', -135, -135, 270, 270)
    ctx.restore()
  ctx.restore()
  ctx.save()
  ctx.transform(1, 0, 0, 1, 1016.5948767945689, 1112.7518828219565)
    ctx.save()
    ctx.transform(1, 0, 0, 1, -5.000000058430487e-7, -43.740781999999996)
      ctx.save()
      ctx.transform(1, 0, 0, 1, 0, -78.4)
      ctx.fillStyle = '#050202'
      ctx.font = 'normal  40px "PingFang TC"'
      ctx.fillText('农', -20, 12.599999999999998, undefined)
      ctx.restore()
      ctx.save()
      ctx.transform(1, 0, 0, 1, 0, -39.2)
      ctx.fillStyle = '#050202'
      ctx.font = 'normal  40px "PingFang TC"'
      ctx.fillText('历', -20, 12.599999999999998, undefined)
      ctx.restore()
      ctx.save()
      ctx.fillStyle = '#050202'
      ctx.font = 'normal  40px "PingFang TC"'
      ctx.fillText('戊', -20, 12.599999999999998, undefined)
      ctx.restore()
      ctx.save()
      ctx.transform(1, 0, 0, 1, 0, 39.19999999999999)
      ctx.fillStyle = '#050202'
      ctx.font = 'normal  40px "PingFang TC"'
      ctx.fillText('戌', -20, 12.599999999999998, undefined)
      ctx.restore()
      ctx.save()
      ctx.transform(1, 0, 0, 1, 0, 78.4)
      ctx.fillStyle = '#050202'
      ctx.font = 'normal  40px "PingFang TC"'
      ctx.fillText('年', -20, 12.599999999999998, undefined)
      ctx.restore()
    ctx.restore()
  ctx.restore()
  ctx.save()
  ctx.transform(1, 0, 0, 1, 60.36585365853671, 1476.72131147541)
    ctx.save()
      ctx.save()
      ctx.transform(1, 0, 0, 1, 0, -112)
      ctx.fillStyle = '#050202'
      ctx.font = 'normal  28px "PingFang TC"'
      ctx.fillText('长', -14, 8.819999999999999, undefined)
      ctx.restore()
      ctx.save()
      ctx.transform(1, 0, 0, 1, 0, -67.2)
      ctx.fillStyle = '#050202'
      ctx.font = 'normal  28px "PingFang TC"'
      ctx.fillText('按', -14, 8.819999999999999, undefined)
      ctx.restore()
      ctx.save()
      ctx.transform(1, 0, 0, 1, 0, -22.400000000000006)
      ctx.fillStyle = '#050202'
      ctx.font = 'normal  28px "PingFang TC"'
      ctx.fillText('扫', -14, 8.819999999999999, undefined)
      ctx.restore()
      ctx.save()
      ctx.transform(1, 0, 0, 1, 0, 22.400000000000006)
      ctx.fillStyle = '#050202'
      ctx.font = 'normal  28px "PingFang TC"'
      ctx.fillText('码', -14, 8.819999999999999, undefined)
      ctx.restore()
      ctx.save()
      ctx.transform(1, 0, 0, 1, 0, 67.19999999999999)
      ctx.fillStyle = '#050202'
      ctx.font = 'normal  28px "PingFang TC"'
      ctx.fillText('开', -14, 8.819999999999999, undefined)
      ctx.restore()
      ctx.save()
      ctx.transform(1, 0, 0, 1, 0, 112)
      ctx.fillStyle = '#050202'
      ctx.font = 'normal  28px "PingFang TC"'
      ctx.fillText('玩', -14, 8.819999999999999, undefined)
      ctx.restore()
    ctx.restore()
  ctx.restore()
ctx.restore()

if (!isInBrowser) {
  const base64 = canvas.toDataURL()
  require('fs').writeFileSync(require('path').join(process.cwd(), 'examples', 'real-template', 'result.png'), base64.split(',')[1], {
    encoding: 'base64'
  })
}
