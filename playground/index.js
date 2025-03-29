let THREAD_REGISTRY = new Map();
let THREAD_COUNTER = 1; // THREAD_ID 0 is reserved to refer to the initial thread

function register_thread(worker_handle) {
    let thread_id = THREAD_COUNTER;

    THREAD_REGISTRY.set(thread_id, worker_handle)
    THREAD_COUNTER = THREAD_COUNTER + 1;

    return thread_id;
}

async function run_wasm() {
    console.log("JScript: loading wasm ...");

    let module = await fetch("playground.wasm")
        .then((response) => response.arrayBuffer())
        .then((wasm) => WebAssembly.compile(wasm));
    let memory = new WebAssembly.Memory({ initial: 17, maximum: 16384, shared: true});

    function spawn_worker(job_id) {
        let thread = new Worker("worker.js");
        thread.addEventListener("message", (event) => {
            let {code, payload} = event.data;
            if (code == 20) {
                let {thread_id, description, lock_id} = payload;
                //console.log("Got event from Thread ", thread_id, ": ", description, lock_id)
            } else if (code == 10) {
                let {thread_id, job_id} = payload;
                let tid = spawn_worker(job_id);
                console.log("Thread ", thread_id, " spawned a new thread ", tid);
            } else {
                console.warn("Unrecognized message code: ", code, ". This callback is now a noop!")
            }
        });
        let tid = register_thread(thread);
        thread.postMessage({module, memory, thread_id: tid, job_id});

        return tid;
    }

    let thread_id = 0;
    let instance = await WebAssembly.instantiate(module, {
        env: { 
            memory: memory
        },
        wasm_ca: {
            thread_spawn: (job_id) => {
                let tid = spawn_worker(job_id);
                console.log("Thread ", thread_id, " spawned a new thread ", tid);
            },
            start_lock: (lock_id) => {
                console.log("Thread ", thread_id, " requested lock ", lock_id);
            },
            finish_lock: (lock_id) => {
                console.log("Thread ", thread_id, " aquired lock ", lock_id);
            },
            start_unlock: (lock_id) => {
                console.log("Thread ", thread_id, " started to release lock ", lock_id);
            },
            finish_unlock: (lock_id) => {
                console.log("Thread ", thread_id, " finished to release lock ", lock_id);
            }
        },
        playground: {
            log_iterations: (n) => {
                console.log("Thread ", thread_id, " did ", n, "iterations!")
            }
        }
    });

    console.log('JScript: index.js loaded.');

    instance.exports.playground_create_deadlock();

    console.log("Main script finished!")
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