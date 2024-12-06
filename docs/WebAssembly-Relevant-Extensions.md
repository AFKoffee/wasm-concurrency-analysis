# Relevant WebAssembly Extensions
This document briefly describes WebAssembly extensions relevant for this project. All of them are standardized already (see status of finished proposals [here](https://github.com/WebAssembly/proposals/blob/main/finished-proposals.md)). Furthermore, all of the following extensions excluding _mutable-globals_ are already part of the WebAssembly 2.0 core specification.

## Related to Multithreading
While _mutable-globals_ have some implications for multithreaded WebAssembly, the most crucial extension for concurrency is _bulk-memory_. The (not yet standardized) _threads_ proposal is described in a separate document.

### Mutable Globals ([Official Overview](https://github.com/WebAssembly/mutable-global/blob/master/proposals/mutable-global/Overview.md))
Introduces the ability to make _globals_, that are imported to or exported from a WebAssembly module mutable. WebAssembly globals are local to the agent, and can therefore not be shared among threads. This makes them a perfect fit for thread local storage.

### Bulk-Memory Operations ([Official Overview](https://github.com/WebAssembly/spec/blob/main/proposals/bulk-memory-operations/Overview.md))
This proposal adds instructions to modify ranges of memory and table entries:
- Memory: Fill, Init, Copy and Drop
- Tables: Fill, Init, Copy and Drop

The most important change of this proposal, however, is the introduction of passive data segments (same addition has also been made for tables). Previously, the given memory of a WebAssembly module was filled with the contents of the data segments during instantiation. This is a problem for shared memory (subject to the _threads_ proposal), because multiple modules would initialize the same memory multiple times possibly overwriting changes in the data. Now, only _active_ data segments are copied over to memory during initialization. Passive data segments have to be initialized _manually_ using the _memory.init_ instruction. This enables conditional initialization of memory shared between agents. 

## LLVM Default Extensions
These extensions are enabled by default in LLVM. Because this project uses Wasm modules compiled from Rust code, which in turn uses LLVM as a compiler backend. These extensions have to be either supported or disabled.

### Multi-value ([Official Overview](https://github.com/WebAssembly/spec/blob/main/proposals/multi-value/Overview.md))
Generalizes the result type of blocks, instructions functions to allow for multiple values. Before that, they were only allowed to return a single value.

Furthermore, blocks are now allowed to have input parameters eliminating the need to use locals for passing values into a block.

### Reference Types ([Official Overview](https://github.com/WebAssembly/spec/blob/main/proposals/reference-types/Overview.md))
Introduces a type of values: _reference types_. These are split into _funcrefs_ and _externrefs_, where funcrefs are references to functions and externrefs are references to arbitrary objects owned by the embedder. Additional instructions have been introduced to create function references or check for null-pointers.

Note, that those reference types are first-class references to objects in the runtime store and can only be stored in tables.

Furthermore, this proposal introduces new table instructions to handle reference types, simplify access and extend its size if necessary.


### Sign-extension Operators ([Official Overview](https://github.com/WebAssembly/spec/blob/main/proposals/sign-extension-ops/Overview.md)) 
Introduces five new integer instructions for sign-extending 8-bit, 16-bit and 32-bit values.