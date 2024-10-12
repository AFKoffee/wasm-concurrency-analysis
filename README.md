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


## Solution Ideas
### 1. Rely on the futex-like feature
Principles:
- Assume, that the `memory.atomic.wait` instruction always targets an address containing a mutex -> i.e. represents waiting to aquire a lock.
- Assume, that the `memory.atomic.notify` instructon always targets an address containing a mutex -> i.e. represents releasing a lock.

By scanning the adresses of those locks in conjunction with the corresponding memory, one can identifiy the different locks. However, a thread does not have to use those instructions to change mutex state - `i32.atomic.rw.cmpxchng` also could.

**Idea**:\
Use *wait* and *notify* to identify the mutex-addresses. Track every change to those addresses assuming *0* to mean unlocked and *1* to mean locked.

### 2. Use global variables to identify threads manually
Because there are no thread spawning or destruction calls to intercept, the current thread could be identified at the first memory access. At the time of this writing, global variables are thread-local, but can be imported and exported (see [mutable-global proposal](https://github.com/WebAssembly/mutable-global/blob/master/proposals/mutable-global/Overview.md)). This means we may be able to use the a non-exported, non-imported global variable to store a unique thread identifier on the first memory access and identify the thread on every later memory access using this id.

**How to get this ID?**\
The module has no knowledge of other threads. The ID could therefore be provided via an imported generate_id() function, that externally keeps track on the number of identified threads.

## Implementation Strategy
There are several approaches to instrument Wasm, which are currently available: 
- [Wasabi](https://github.com/danleh/wasabi/tree/master) (Paper [here](https://arxiv.org/pdf/1808.10652)): Written in Rust, instruments instructions with many low-level javascript hooks. **Problem**: No support for [threads proposal](https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md), [multi-memory proposal](https://github.com/WebAssembly/multi-memory/blob/main/proposals/multi-memory/Overview.md)
- [Fuzzm](https://github.com/fuzzm/fuzzm-project) (Paper [here](https://arxiv.org/pdf/2110.15433)): WebAssembly fuzzer. Does not help with deadlock detection, but acts as an inspiration for insturmenting Wasm in general.
- [BREWasm](https://github.com/security-pride/BREWasm/tree/main) (Paper [here](https://arxiv.org/pdf/2305.01454)): Written in Python, general purpose Wasm rewriting framework. **Problem**: No support for [threads proposal](https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md), [multi-memory proposal](https://github.com/WebAssembly/multi-memory/blob/main/proposals/multi-memory/Overview.md)

Because no approach really already satisfies the requirements. The following approach seems to be the most promising:

1. Use a SOTA actively maintained parser:\
The [wasmparser](https://github.com/bytecodealliance/wasm-tools/tree/main/crates/wasmparser) library is a *Bytecode Alliance* project implementing many modern Wasm proposals including the [threads proposal](https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md) and [multi-memory proposal](https://github.com/WebAssembly/multi-memory/blob/main/proposals/multi-memory/Overview.md). This is also the parser used in the *Wasabi-Framework*, but the framework itself does not support the proposals.
2. Use a tool to encode parsed components back to Wasm:
The [wasm-encoder](https://github.com/bytecodealliance/wasm-tools/tree/main/crates/wasm-encoder) as also a *Bytecode Alliance* project can produce valid WebAssembly binaries programmatically. Because both crates are developed and maintained by the same organization, *wasmparser* structures can be converted to *wasm-encoder* structs with low effort (or at least the conversion code can be viewed in github if customization is needed)
3. Implement a middleware to instrument the Wasm code by parsing it with *wasmparser*, adding desired functionality to the parsed Wasm module and encoding it back to binary with *wasm-encoder*. This step could be inspired by existing solutions like the ones above.
4. Runtime Integration: Instead of encoding the instrumented Wasm back to file, running e. g. *wasmtime* on the generated wasm-bytes directly could be possible making the tool easier to use.
5. Alternative: Hack the *wasmtime* runtime to instrument the code before or during the compilation process. This would avoid parsing the code twice, but is it really possible or worth it???
6. Better thread and mutex support: As the thread-management and implementation of mutex-like structures is deferred to the embedder of Wasm, getting reliable information about the creation and destruction of threads as well as the use of mutex-like structures has to take into account the embedder. This could be [emscripten](https://github.com/emscripten-core/emscripten) or a nightly-compiled Rust program (see [this example](https://rustwasm.github.io/wasm-bindgen/examples/raytrace.html?highlight=thread#parallel-raytracing)). Those compilation mechanisms may use different conventions to translate thread-operations to Wasm and therefore need different instrumentation support.

Long-term arguments for the *Bytecode Alliance* libraries:
 - The organization is backed by big vendors like *Mozilla*, *Amazon*, *Anaconda*, *Docker* and more
 - The organization is non-profit and works open-source
 - --> The libraries are likely to be maintained long-term and to incorporate new standardized features and proposals fast making them a reliable fit for this project.