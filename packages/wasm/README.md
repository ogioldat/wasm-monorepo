# WASM

This package will contain the WASM binary.


## Compilation

To compile the binary run the following in `apps/rust-wasm`

```shell
wasm-pack build -d ../../packages/wasm --target web --out-name wasm
```

## Usage

With `pnpm`'s workspaces on place you can use WASM binary like this:

```js
import init from 'wasm'
```

