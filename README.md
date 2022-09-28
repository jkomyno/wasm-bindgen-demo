# wasm-bindgen demo

We have two example flavors of a demo `getConfig` function:
- `getConfigDeserializeInput`: "Deserialized" version of the `getConfig` function, where the input is provided as a JSON string.
It panics if the input is not a valid JSON string compatible with the `GetConfigParams` type
- `getConfigTyped`: "Typed" version of the `getConfig` function, where the function input is provided as a `GetConfigParams` object directly.

## Build Rust crates

- Open a terminal in the [`./rust`](./rust) folder
- Install `wasm-bindgen@0.2.83`

```bash
cargo update -p wasm-bindgen 
cargo install -f wasm-bindgen-cli@0.2.83
```

- Build the Rust libraries and the WebAssembly artifacts

```bash
chmod +x ./build.sh
OUT=../nodejs ./build.sh
```

## Run Node.js examples

- Open a terminal in the [`./nodejs`](./nodejs) folder
- Install the dependencies

```bash
npm i
```

#### Run the "Deserialized" version

```bash
npm run deserialized
```

Example output:

```bash
value@getConfigDeserializeInput: ok

panic@getConfigDeserializeInput: RuntimeError: unreachable
    at __rust_start_panic (wasm://wasm/00096816:wasm-function[331]:0x1c376)
    at rust_panic (wasm://wasm/00096816:wasm-function[207]:0x1baaa)
    at std::panicking::rust_panic_with_hook::hb09154fa23e06c37 (wasm://wasm/00096816:wasm-function[124]:0x196c4)
    at std::panicking::begin_panic_handler::{{closure}}::h6091c197f0d08bf0 (wasm://wasm/00096816:wasm-function[141]:0x1a313)
    at std::sys_common::backtrace::__rust_end_short_backtrace::h004afb3e6a867c40 (wasm://wasm/00096816:wasm-function[247]:0x1bf71)
    at rust_begin_unwind (wasm://wasm/00096816:wasm-function[197]:0x1b8c4)
    at core::panicking::panic_fmt::h9e229748e3ae9f9d (wasm://wasm/00096816:wasm-function[199]:0x1b945)
    at get_config::get_config_deserialize_input::h7b785da7ff3fbf0b (wasm://wasm/00096816:wasm-function[42]:0x1133c)
    at getConfigDeserializeInput (wasm://wasm/00096816:wasm-function[74]:0x15861)
    at module.exports.getConfigDeserializeInput (/Users/jkomyno/work/prisma/wasm-bindgen-demo/nodejs/src/wasm/get_config_wasm.js:125:14)

is error a panic?  true

Fin
```

#### Run the "Typed" version

```bash
npm run typed
```

Example output:

```bash
value@getConfigTyped: ok

error@getConfigTyped: Error: {"errorCode":"P1012","message":"This is an error"}
    at module.exports.__wbindgen_error_new (/Users/jkomyno/work/prisma/wasm-bindgen-demo/nodejs/src/wasm/get_config_wasm.js:184:17)
    at getConfigTyped (wasm://wasm/00096816:wasm-function[34]:0xf71c)
    at module.exports.getConfigTyped (/Users/jkomyno/work/prisma/wasm-bindgen-demo/nodejs/src/wasm/get_config_wasm.js:151:14)
    at main (/Users/jkomyno/work/prisma/wasm-bindgen-demo/nodejs/src/typed.ts:20:19)
    at Object.<anonymous> (/Users/jkomyno/work/prisma/wasm-bindgen-demo/nodejs/src/typed.ts:32:1)
    at Module._compile (node:internal/modules/cjs/loader:1105:14)
    at Module.m._compile (/Users/jkomyno/work/prisma/wasm-bindgen-demo/nodejs/node_modules/ts-node/src/index.ts:1618:23)
    at Module._extensions..js (node:internal/modules/cjs/loader:1159:10)
    at Object.require.extensions.<computed> [as .ts] (/Users/jkomyno/work/prisma/wasm-bindgen-demo/nodejs/node_modules/ts-node/src/index.ts:1621:12)
    at Module.load (node:internal/modules/cjs/loader:981:32)

errorOutput@getConfigTyped: { errorCode: 'P1012', message: 'This is an error' }

is error a panic?  false

Fin
```