# Thread Local Storage and Linking
The absence of a standardized way to create and manage threads in WebAssembly makes it difficult to create a trace of events containing `spawn`, `join`, `lock` and `release` events. While there is already _synchronization_ support in the language, there is no specified way for runtimes to initialize Thread Local Storage and a Thread Local Stack Space. Therefore, blindly compiling multi-threaded code to WebAssembly and intializing it in WebWorkers with the same shared memory will lead to memory errors as the different threads will most likely overwrite their (shadow) stack values.

## Wasm-lld Linker Symbols
If shared memory is enabled at compile time, the llvm-linker for wasm emits a number of symbols to be picked up by runtimes / instrumentation tools. These symbols indicate relevant information for multi-threaded programs (see: [Tool Conventions for Linking](https://github.com/WebAssembly/tool-conventions/blob/main/Linking.md#experimental-threading-support)).

Note: That the tool-convention does only cover memory initialization and thread local storage. The llvm compiler backend also emits code, which operates on an __explicit__ stack (in addition to the implicit WebAssembly stack). Care has to be taken to allocate such a stack upon each creation of a thread, i.e. in the start section.

See wasm-lld linker symbols here: https://github.com/llvm/llvm-project/blob/main/lld/wasm/Symbols.h#L541

## Wasm-lld Linker Memory Layout
By default, the wasm-lld linker emits the following memory layout:
```
-------------------------------------------------------------
|               |                   |                       |
|  Static Data  | <==== Call Stack  |   Heap ==========>    |
|               |                   |                       |
-------------------------------------------------------------
```

Rust uses the `stack first` option to emit this memory layout:
(See: https://github.com/rust-lang/rust/blob/master/compiler/rustc_target/src/spec/base/linux_wasm.rs#L33)
```
-------------------------------------------------------------
|                   |                 |                     |
|  <==== Call Stack |   Static Data   | Heap ==========>    |
|                   |                 |                     |
-------------------------------------------------------------
```

Symbols defining the end of the sections are:

- __data_end: Marking the and of the data and bss
- __heap_base: Marking the beginning of the heap

The rust compiler expors some of them by default: https://github.com/rust-lang/rust/blob/master/compiler/rustc_codegen_ssa/src/back/linker.rs#L1439

## Wasm-Bindgen and Multithreading
Wasm bindgen prepares a compiled WebAssembly module for multithreading by injecting a custom function into the start section, which will conditionally execute the linkers' memory initialization function to make sure, that memory is only initialized once. Additionally, it allocates additional memory to provide thread local storage at startup for each thread and provides each thread with a separate stack by adjusting the stack pointer to point to thread-exclusively allocated memory just for that purpose. 

For details refer to the source code here: https://github.com/rustwasm/wasm-bindgen/blob/main/crates/threads-xform/src/lib.rs

Note: Wasm-Bindgen is intended to prepare a WebAssembly module that has been compiled from Rust for usage on the web. It is therefore not suited to be used with native runtimes.

## Specifying Wasm Exports/Imports from Rust
[This article](https://surma.dev/things/rust-to-webassembly/) explains the basics of emitting a Wasm-Module compiled from Rust that imports and exports specified functions. The used ABI is specified [here](https://github.com/WebAssembly/tool-conventions/blob/main/BasicCABI.md).

## Thread Handling for Multithreaded WebAssembly
1. On the Web:  
Use `wasm-bindgen` and the provided mechanisms to interact with the browser APIs. As there is most likely the need to link against a custom threading and syncronization library, we might as well ship thread management for the web with it.  
Proposed Mechanism:  
Instead of relying on imports expose a listener API, which allows external JavaScript to pass function handles to the library, which in turn calls these functions upon specific events (spawn, join, ...). We could expose an additional function for _memory-access-events_ which could then be called by Wasabi instrumentation directly removing the need to call JavaScript hooks (exposing this function could also make sense if external JavaScript might modify shared memory).  
Advantage: Trace generation could fully happen inside the WebAssembly Module enabling it to syncronize adequately over the Trace data structure as this struct lies in shared memory.
2. Using native runtimes:
Can be quite challenging as we dont know how the linker-generated `__wasm_init_memory` function works in detail (i.e. does it initialize memory twice if called twice or is the prevented by the linker already) ==> No: here is the code: https://github.com/llvm/llvm-project/blob/00cb966209955878cee903068333d4d2d4f7b259/lld/wasm/Writer.cpp#L1222. Furthermore, Thread Local Storage as well as a Thread Local Stack would have to be allocated manually (although we might be able to utilize wbgs' patching abilities without importing functions with wasm-bindgen because these functions would panic unconditionally as described [here](https://rustwasm.github.io/wasm-bindgen/reference/rust-targets.html)). If we want to include a web-independent thread management into a Wasm-Module, we would have to patch it like wasm-bindgen does.  
Alternatively, it might be possible to patch the module from inside by accessing wasm-lld linker symbols from Rust code ???!!
Like this: https://stackoverflow.com/questions/72820626/how-to-access-a-variable-from-linker-script-in-rust-code
==> No: Look this comment: https://github.com/emscripten-core/emscripten/issues/12489#issuecomment-707974350. Symbols of type global can not be referenced from source code (its C++ but should apply similarly to rust)