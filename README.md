# Wasm Concurrency Analysis

Experimental project trying to instrument WebAssembly (Wasm) modules to perform rudimentary deadlock or datarace detection.

## Fundamental Goals
Since the [threads proposal](https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md) has been implemented in most web browsers ([see here](https://developer.mozilla.org/en-US/docs/WebAssembly#browser_compatibility)) and SOTA Wasm runtimes (e.g. [wasmtime](https://github.com/bytecodealliance/wasmtime) and [wasmer](https://github.com/wasmerio/wasmer)), concurrent access of memory, shared by different Wasm module instances, became possible. A way to detect dataraces or deadlocks without having to inspect the source code, which was compiled to WebAssembly, would therefore come in handy.

**Idea**: Instrument the Wasm binary to generate a trace of locking actions and memory accesses, which can then be analyzed directly at runtime or saved for a later, more in-depth analysis.


## Problems
### 1. Mutex structures are implementation dependent
The Wasm specification only provides the tools to atomically access memory and simple futex-like instructions, which can be used to *implement* locks, but they can not be directly detected by simply analyzing the Wasm module (see [threads proposal](https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md)).

### 2. Thread handling is embedder dependent
The Wasm specification doesn't provide functionality to create or join threads as well as the ability to identify the current thread in any way. This is deferred to the embedder (see [threads proposal](https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md)).


## Solution Ideas
### 1. Rely on the futex-like feature
Principles:
- Assume, that the `memory.atomic.wait` instruction always targets an address containing a mutex -> i.e. represents waiting to aquire a lock.
- Assume, that the `memory.atomic.notify` instructon always targets an address containing a mutex -> i.e. represents releasing a lock.

By scanning the adresses of those locks in conjunction with the corresponding memory, one can identifiy the different locks. However, a thread does not have to use those instructions to change mutex state - `i32.atomic.rw.cmpxchng` also could.

**Idea**:\
Use *wait* and *notify* to identify the mutex-adresses. Track every change to those adresses assuming *0* to mean unlocked and *1* to mean locked.

### 2. Use global variables to identify threads manually
Because there are no thread spawning or destruction calls to intercept, the current thread could be identified at the first memory access. At the time of this writing, global variables are thread-local, but can be imported and exported (see [mutable-global proposal](https://github.com/WebAssembly/mutable-global/blob/master/proposals/mutable-global/Overview.md)). This means we may be able to use the a non-exported, non-imported global variable to store a unique thread identifier on the first memory access and identify the thread on every later memory access using this id.

**How to get this ID?**\
The module has no knowledge of other threads. The ID could therefore be provided via an imported generate_id() function, that externally keeps track on the number of identified threads.

## Implementation Strategy
...