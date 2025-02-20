
const {create_deadlock} = wasm_bindgen;

let THREAD_ID = 0;

let utils = {
    register_new_thread: function () {
        let id = THREAD_ID;
        THREAD_ID += 1;
        return id.toString();
    }
}

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

let file_button = document.getElementById("filetest");
file_button.onclick = async _event => {
    const file = new File(["this is an example file"], "foo.txt", {
        type: "text/plain",
    });

    let a = document.getElementById("invisible-download-link");
    if (!a) {
        a = document.createElement('a');
        a.id = "invisible-download-link";
    }
    
    a.href = URL.createObjectURL(file);
    a.download = file.name;
    document.body.appendChild(a);
    a.click();
}