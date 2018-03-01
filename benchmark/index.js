const reddir = require('readdir-enhanced')
const { join } = require('path')
const microtime = require('microtime')
const { range } = require('lodash')

const cario = require('./cario')
const skia = require('./skia')

const sleep = time =>
  new Promise(resolve => {
    setTimeout(resolve, time)
  })

reddir.sync(join(process.cwd(), 'benchmark', 'fixtures'))
  .forEach(async file => {
    const func = require(`./fixtures/${ file }`)
    const startCario = microtime.now()
    await Promise.all(range(100).map(() => {
      return func(cario)
    }))
    const carioTime = microtime.now() - startCario

    await sleep(5000)

    const startSkia = microtime.now()
    await Promise.all(range(100).map(() => {
      return func(skia)
    }))
    const skiaTime = microtime.now() - startSkia
    const type = file.split('.')[0]
    console.info(`Cario run ${ type } cost: ${ carioTime } ms`)
    console.info(`Skia  run ${ type } cost: ${ skiaTime } ms`)
  })
