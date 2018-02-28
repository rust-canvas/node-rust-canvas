const os = require('os')
const { join } = require('path')
const child_process = require('child_process')
const { copyFileSync } = require('fs')

const platform = os.platform()

const args = ['--', '-C', 'link-args=-Wl,-undefined,dynamic_lookup']

let env = 'debug'

if (process.env.NODE_ENV === 'production') {
  args.unshift('--release')
  env = 'release'
}

child_process.spawnSync(
  'cargo',
  ['rustc', ...args],
  {
    cwd: process.cwd(),
    stdio: 'inherit',
    env: {
      ...process.env,
      NEON_NODE_ABI: process.versions.modules,
    },
  }
)

let fileExt = ''

switch (platform) {
  case 'darwin':
    fileExt = 'dylib'
    break
  case 'freebsd':
  case 'linux':
  case 'sunos':
    fileExt = 'so'
    break
  case 'win32':
    fileExt = 'dll'
    break
  default:
    throw new TypeError('unkonwn platform')
}

const dylibFile = join(process.cwd(), 'target', env, `libnode_rust_canvas.${ fileExt }`)

console.info(`copy ${ dylibFile }`)

copyFileSync(dylibFile, './index.node')
