import { getConfigDeserializeInput } from './wasm/get_config_wasm'
import type { GetConfigParams } from './wasm/get_config_wasm'
import { isWasmPanic } from './isWasmPanic'

/**
 * "Deserialized" version of the `getConfig` function, where the input
 * is provided as a JSON string.
 */
async function main() {
  const params: GetConfigParams = {
    prismaSchema: 'datasource db { provider = "sqlite" url = "file:dev.db" }',
  }

  const value = getConfigDeserializeInput(JSON.stringify(params))
  console.log('value@getConfigDeserializeInput:', value)

  // providing inputs that don't serialize to `GetConfigParams`
  // should result in a panic in Rust
  try {
    getConfigDeserializeInput('{}')
  } catch (e) {
    const error = e as Error
    console.log('\npanic@getConfigDeserializeInput:', error)
    console.log('\nis error a panic? ', isWasmPanic(error))
  }

  console.info('\nFin')
}

main()
