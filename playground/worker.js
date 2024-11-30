importScripts('pkg/playground.js');

console.log("Initializing standalone worker")

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

    self.onmessage = async event => {
        await initialised;
        wasm_bindgen.worker_entry_point(event.data)
    };
}