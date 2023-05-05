# WASM learning project

These are just a rough sketches around building WASM monorepo apps.
Expect a lot of areas for improvements :P 


## Prerequisites

- `rust`
- `cargo`
- `wasm-pack` `cargo install wasm-pack`

## Build the WASM binary

```shell
cd apps/rust-wasm && wasm-pack build -d ../../packages/wasm --target web --out-name wasm
```

## TODOS

1. Use nx for wasm build?
