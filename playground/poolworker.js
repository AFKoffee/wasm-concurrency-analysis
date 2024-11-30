importScripts('pkg/playground.js');

console.log("Initializing pool worker")

self.onmessage = event => {
  // event.data in this case is an array [module, memory]
  // __wbg_init is the default export from "playground.js" it expects a module and a memory and instanciates the wasm
  let initialised = wasm_bindgen(...event.data).catch(err => {
    // Propagate to main `onerror`:
    setTimeout(() => {
      throw err;
    });
    // Rethrow to keep promise rejected and prevent execution of further commands:
    throw err;
  });

  // Replaces the last on-message callback with the new callback that expects a function to execute (a Work struct)
  self.onmessage = async event => {
    // This will queue further commands up until the module is fully initialised:
    await initialised;
    wasm_bindgen.child_entry_point(event.data); // Calls the javascript function with a work struct (as raw pointer)
  };
};