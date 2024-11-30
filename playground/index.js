
const {create_deadlock} = wasm_bindgen;

async function run_wasm() {
    console.log("JScript: loading wasm bindings ...");

    await wasm_bindgen();

    console.log('JScript: index.js loaded.');

    create_deadlock();
}

let button = document.getElementById("deadlock");

button.onclick = _event => {
    run_wasm();
}