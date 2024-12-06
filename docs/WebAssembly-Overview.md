# WebAssembly Overview
This document aims to give a short overview regarding the WebAssembly 
core standard. It provides the necessary information to understand WebAssembly modules, 
memory management, module instantiation and the interaction with the host environment 
on a high level. The information is mainly taken from the _WebAssembly 1.0 Core Specification_ 
(also known as the _MVP_), which can be consulted for in-depth details.

## Wasm Modules
A WebAssembly _module_ is the unit of deployment, loading and compilation and bundles 
the following information:
- Types: A vector of all function type signatures used in the module
- Functions: A vector of all function definitions of the module
- Tables: A vector defining all tables in the module
- Memories: A vector defining all memories in the module
- Globals: A vector defining all global variables of the module
- Element Segments: A vector of initialization instructions for tables
- Data Segments: A vector of initialization instructions for memories
- Start: An optional start function (executed after module instantiation)
- Imports: A list of functions imported from the host environment
- Exports: A list of module functions made available to the host environment


## Memory Management
A WebAssembly module can define _(linear) memories_. These are contiguous,
mutable arrays of raw bytes, which can be grown dynamically. Note, that every access
to memory will be bounds-checked. In the WebAssembly MVP, **at most one**
memory may be defined or imported in a single module (changes, if the _multi-memory_ proposal
is supported).

There are the following groups of memory instructions in WebAssembly 1.0:
- Load: Retrieves a value from memory using a dynamic _base adress_, _offset_ and _alignment_. Exists in various versions depending on the values' bitwidth.
- Store: Saves a value to memory using a dynamic _base adress_, _offset_ and _alignment_. Exists in various versions depending on the values' bitwidth.
- Size: Returns the size of the memory in pages (multiples of 64 KiB)
- Grow: Increases the size of the memory by the given number of pages. 

## The Host Environment (Embedder)
The _host environment_ (or _embedder_) is responsible for WebAssembly 
execution according to the specification. This includes the management 
and creation of different module instances, the allocation of physical
memory and correct representation of the abstract stack machine at runtime.
In contemporary Wasm runtimes (e.g. SpiderMonkey from Firefox), the WebAssembly
code may be JIT compiled to native machine code. To use CPU registers in 
an optimal way, the abstract stack machine is carefully translated into
register based machine instructions, that resemble the original behavior.
WebAssembly can therefore also be seen as a low-level intermediate language.

Furthermore, the embedder can interact with the WebAssembly code through imported 
or exported functions. As briefly stated in [Wasm Modules](#wasm-modules), a 
module imports functions from the host environment. These functions have to 
be provided during execution. Once instantiated, the module also exports 
functions to the host environment. These can be called by external code 
(e.g. JavaScript in the browser).

## Execution
The execution of WebAssembly code is based on three important components:
The _store_, the _stack_ and _module instances_.

### The Stack
WebAssembly instructions interact with an implicit stack. The stack keeps track
of _operands_, _labels_, and _activations_:
- Operands: Simple values, which are popped of the stack during instruction execution.
- Labels: Active control instructions, that can be targeted by branching instructions. E.g. Loops, Blocks, ...
- Activations: Call frames of active function calls. Local variables defined in the function are only valid inside this scope.

### The Store
The store represents all global state that can be manipulated by WebAssembly programs.
It holds instances of the following elements:
- Functions
- Tables
- Memories
- Global variables

Note that all instantiated modules interact with this store and do not have mutable
states themselves. The store is managed by the host environment and WebAssembly programs
only interact with it indirectly.

### Module Instantiation
_Module Instantiation_ is the process of creating a runtime representation for a given
WebAssembly module. A _module instance_ collects all runtime representations of the
entities that are imported, defined or exported by the module.

**Note:** These runtime representations are put into the store and are only _referenced_
by the module instance via indices into the respective store component. This will especially
important, when talking about sharing memory between modules.

### Invocation
After a module has been instanciated. The host environment can invoke exported functions
of the module to access the provided functionality. Note that calling an exported
function leads to the invocation of a runtime function representation in the global
store, accessed via its function adress.