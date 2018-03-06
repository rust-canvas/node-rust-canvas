const chalk = require('chalk')
const reddir = require('readdir-enhanced')
const { join } = require('path')
const microtime = require('microtime')
const { range } = require('lodash')

const cario = require('./cairo')
const skia = require('./skia')

const sleep = time =>
  new Promise(resolve => {
    setTimeout(resolve, time)
  })

const showMsg = msg => console.info(chalk.bgBlack(chalk.cyan(msg)))

reddir.sync(join(process.cwd(), 'benchmark', 'fixtures'))
  .reduce(async (acc, file) => {
    await acc
    const func = require(`./fixtures/${ file }`)
    const type = file.split('.')[0]

    showMsg(`Benchmark ${ type } start`)

    const startCairo = microtime.now()
    await Promise.all(range(100).map(() => {
      return func(cario)
    }))
    const cairoTime = microtime.now() - startCairo

    await sleep(5000)

    const startSkia = microtime.now()
    await Promise.all(range(100).map(() => {
      return func(skia)
    }))
    const skiaTime = microtime.now() - startSkia
    showMsg(`Cairo run ${ type } cost: ${ cairoTime } ms`)
    showMsg(`Skia  run ${ type } cost: ${ skiaTime } ms`)
  }, Promise.resolve())
