//importScripts('pkg/playground_bg.wasabi.js')
//importScripts('pkg/analysis.js');
importScripts('pkg/playground.js');

console.log("JScript: initializing standalone worker")

// Wait for the main thread to send us the shared module/memory. Once we've got
// it, initialize it all with the `wasm_bindgen` global we imported via
// `importScripts`.
//
// After our first message all subsequent messages are an entry point to run,
// so we just do that.
self.onmessage = event => {
    let initialised = wasm_bindgen(...event.data).catch(err => {
        // Propagate to main `onerror`:
        setTimeout(() => {
          throw err;
        });
        // Rethrow to keep promise rejected and prevent execution of further commands:
        throw err;
    });

    console.log("JScript: first message sent to worker. Initializing wasm module and replacing onmessage-callback.")

    self.onmessage = async event => {
        await initialised;
        console.log("JScript: second message sent to worker. Ready to execute the provided workload.")
        wasm_bindgen.worker_entry_point(event.data)
    };
}