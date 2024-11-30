
const {create_deadlock} = wasm_bindgen;

async function run_wasm() {
    console.log("Instanciating wasm");

    await wasm_bindgen();

    console.log('index.js loaded');

    create_deadlock();
}

let button = document.getElementById("deadlock");

button.onclick = _event => {
    run_wasm();
}