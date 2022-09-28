import { getConfigTyped } from './wasm/get_config_wasm'
import type { GetConfigParams } from './wasm/get_config_wasm'
import { isWasmPanic } from './isWasmPanic'

/**
 * "Typed" version of the `getConfig` function, where the function
 * input is provided as a `GetConfigParams` object directly.
 */
async function main() {
  const params: GetConfigParams = {
    prismaSchema: 'datasource db { provider = "sqlite" url = "file:dev.db" }',
    env: {},
    triggerError: false, // when true, this will cause an error in Rust
  }

  const value = getConfigTyped(params)
  console.log('value@getConfigTyped:', value)

  try {
    getConfigTyped({ ...params, triggerError: true })
  } catch (e) {
    const error = e as Error
    const errorOutput = JSON.parse(error.message)
    console.log('\nerror@getConfigTyped:', error)
    console.log('\nerrorOutput@getConfigTyped:', errorOutput)
    console.log('\nis error a panic? ', isWasmPanic(error))
  }

  console.info('\nFin')
}

main()
