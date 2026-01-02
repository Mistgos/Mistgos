# MISTGOS TECHNOLOGY STACK

This document outlines the core technologies and toolchains required to develop and run **MISTGOS**, the progenitor of the **GMOS** lineage.

## 1. Primary Programming Language: Rust

**Why Rust?**
* **Memory Safety:** Prevents common low-level bugs without the overhead of a garbage collector.
* **Modern Tooling:** Leverages `Cargo` for dependency management.
* **Ownership Model:** Naturally complements the **Object-Capability (OCAP)** security model.

## 2. Execution Environment: WebAssembly (WASM)

**Why WASM?**
* **Sandboxing:** Logic Nodes run in a strictly isolated environment.
* **Architecture Neutrality:** Compiled bytecode is portable across x86, ARM, and RISC-V.
* **Language Agnosticism:** Future modules can be written in any language that targets WASM (Zig, C, C++, etc.).

## 3. Development Requirements

To contribute to or build MISTGOS, the following environment is required:

* **Toolchain:** Rust `nightly`.
* **Components:** `rust-src`, `llvm-tools-preview`.
* **Emulation:** **QEMU** (`qemu-system-x86_64`) for testing the kernel and graph state.
* **Binary Utilities:** Tools for creating bootable images (e.g., `xorriso`, `mtools`).

## 4. Architecture Foundation

* **Security Model:** Object-Capability (OCAP).
* **State Management:** Unified directed graph.
* **Execution Engine:** **PULSE** (Parallel Unit Logical Scheduling Engine).

| Layer | Technology | 
| ----- | ----- | 
| **Language** | Rust (Nightly) | 
| **Runtime** | WebAssembly (WASM) | 
| **Build System** | Cargo | 
| **Emulator** | QEMU | 
| **Security** | Object-Capability (OCAP) | 

---

> **Note:** This project is in a pre-alpha stage. The stack is optimized for experimental OS architecture research.
