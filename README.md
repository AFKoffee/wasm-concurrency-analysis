# Wasm Concurrency Analysis

Experimental project trying to instrument WebAssembly (Wasm) modules to perform rudimentary deadlock or datarace detection.

## Fundamental Goals
Since the [threads proposal](https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md) has been implemented in most web browsers ([see here](https://developer.mozilla.org/en-US/docs/WebAssembly#browser_compatibility)) and SOTA Wasm runtimes (e.g. [wasmtime](https://github.com/bytecodealliance/wasmtime), [wasmer](https://github.com/wasmerio/wasmer) and [wamr](https://github.com/bytecodealliance/wasm-micro-runtime)), concurrent access of memory, shared by different Wasm module instances, became possible. A way to detect dataraces or deadlocks without having to inspect the source code, which was compiled to WebAssembly, would therefore come in handy.

**Idea**: Instrument the Wasm binary to generate a trace of locking actions and memory accesses, which can then be analyzed directly at runtime or saved for a later, more in-depth analysis.


## Problems
### 1. Mutex structures are implementation dependent
The Wasm specification only provides the tools to atomically access memory and simple futex-like instructions, which can be used to *implement* locks, but they can not be directly detected by simply analyzing the Wasm module (see [threads proposal](https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md)).

### 2. Thread handling is embedder dependent
The Wasm specification doesn't provide functionality to create or join threads as well as the ability to identify the current thread in any way. This is deferred to the embedder (see [threads proposal](https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md)).

## Related Projects
There are several approaches to instrument Wasm. Some of them are: 
- [Wasabi](https://github.com/danleh/wasabi/tree/master) (Paper [here](https://arxiv.org/pdf/1808.10652)): Written in Rust, instruments instructions with many low-level javascript hooks. **Problem**: No support for [threads proposal](https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md)
- [WasmR3](https://github.com/sola-st/wasm-r3) (Paper [here](https://arxiv.org/pdf/2409.00708)): Record-Reduce-Replay framework for standalone WebAssembly benchmarking. This project is based on Wasabi.
- [Fuzzm](https://github.com/fuzzm/fuzzm-project) (Paper [here](https://arxiv.org/pdf/2110.15433)): WebAssembly fuzzer. Acts as an inspiration for instrumenting Wasm in general.
- [BREWasm](https://github.com/security-pride/BREWasm/tree/main) (Paper [here](https://arxiv.org/pdf/2305.01454)): Written in Python. This is a general purpose Wasm rewriting framework.

## Solution Idea
Extend the Wasabi framework to support the required proposals for multithreaded WebAssembly. Then utilize the framework to instrument the Wasm binary and use the inserted hooks to create a trace of important events (memory accesses, locks, unlocks, ...). Run deadlock- or datarace-detection algorithms on the trace to identify concurrency-related bugs in the original binary.

## Rough Roadmap
- [x] Create an example program that deadlocks on execution using WebWorkers and Wasm

- [x] Write a document showing how the Wasm proposals, which are relevant for this project, relate to each other. (Why do we need support for them? What functionality do they provide?)

- [ ] Extend the Wasabi framework to support the relevant proposals in order to make Wasm binary instrumentation possible for our purposes.

- [ ] Examine the deadlocking examples to identify the parts in the Wasm module, which have to be instrumented for useful execution tracing.

- [ ] Create a trace using the hooks from the Wasabi instrumentation.

- [ ] Perform deadlock detection on the trace.
