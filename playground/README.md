# Wasm Deadlock Detection Playground
This crate serves as a playground to create deadlocks through WebWorkers and Wasm modules with shared memory.

## Getting started
1. Make sure to have the Rust programming language installed (rustc and cargo)
2. Make sure to have the `wasm-pack` command line tool installed
3. Make sure to have python installed (needed for local web server)
4. To build the project execute `wasm-pack build --target no-modules` in the project root directory
5. Start the python web server `python server.py` also from the project root directory
6. Navigate to `localhost:8000` in your browser to see the demo. (Important: Use _localhost_ in the URL. Otherwise the http-headers needed for shared wasm-memory may not work correctly - tested on Firefox)