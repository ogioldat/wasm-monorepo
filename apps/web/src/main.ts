import init from 'wasm'
init().then(wasm => {
    console.log(
        wasm.add_n(2,3)
    )
})