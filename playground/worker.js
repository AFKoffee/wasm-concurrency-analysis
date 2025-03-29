importScripts('playground.wasabi.js')
importScripts('analysis.js');

console.log("JScript: initializing standalone worker")

function build_import_object(memory, thread_id) {
    return {
        env: { 
            memory: memory
        },
        wasm_ca: {
            thread_spawn: (job_id) => {
                self.postMessage({code: 10, payload: {thread_id, job_id}})
            },
            start_lock: (lock_id) => {
                self.postMessage({code: 20, payload: {thread_id, description: "Request Lock", lock_id}})
            },
            finish_lock: (lock_id) => {
                self.postMessage({code: 20, payload: {thread_id, description: "Aquired Lock", lock_id}})
            },
            start_unlock: (lock_id) => {
                self.postMessage({code: 20, payload: {thread_id, description: "Started Release", lock_id}})
            },
            finish_unlock: (lock_id) => {
                self.postMessage({code: 20, payload: {thread_id, description: "Finished Release", lock_id}})
            }
        },
        playground: {
            log_iterations: (n) => {
                console.log("Thread ", thread_id, " did ", n, "iterations!")
            }
        }
    }
}

// Wait for the main thread to send us the shared module/memory. Once we've got
// it, initialize it all with the `wasm_bindgen` global we imported via
// `importScripts`.
self.onmessage = async event => {
    let {module, memory, thread_id, job_id} = event.data;
    console.log("JScript: Ready to execute workload for thread ", thread_id);
    let importObject = build_import_object(memory, thread_id);
    let instance = await WebAssembly.instantiate(module, importObject);
    instance.exports.wasm_ca_thread_entrypoint(job_id);
    console.log("JScript: Worker ", thread_id, " finished its job!");
}