# Wasabi Extension: Implementing Thread Support
This document aims to give a rough overview on how the newly supported extensions integrate with existing Wasabi concepts. 

## WebAssembly 2.0 Spec
The [Wasm-R3 Framework](https://github.com/sola-st/wasm-r3) already uses an [extended Wasabi](https://github.com/doehyunbaek/wasabi) framework, that supports the Wasm 2.0 spec (apart from the SIMD extension). We also don't need SIMD instructions, which means we can implement support for necessary extensions similarly.

## Threads proposal
TODO: Describe how to support shared memory and atomics in Wasabi
