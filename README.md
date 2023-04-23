# WASM learning project


## Prerequisites

- `rust`
- `cargo`
- `wasm-pack` `cargo install wasm-pack`

## Build the WASM binary

```shell
cd packages/rust && wasm-pack build -d ../wasm --target web --out-name wasm
```

## TODOS

1. Use nx for delegating wasm build?