# MISTGOS ARCHITECTURE (GMOS)

This document provides a technical deep dive into the architecture of **MISTGOS**, the progenitor of the **GMOS** (Graph-based Modular Operating System) lineage.

## 1. The Core Split: Engine vs. Shard

To ensure maximum flexibility and the ability for others to create their own "distros," the system is strictly divided into two layers:

### 1.1 The MISTGOS Engine (The "Kernel")
The engine is the heart of the system. It is purely technical and agnostic to the user's intent.
* **Responsibilities:**
    * **Graph Management:** Creating/deleting nodes and edges.
    * **OCAP Enforcement:** Checking if an edge exists before allowing an operation.
    * **PULSE Executor:** Running WASM logic nodes.
    * **Persistence:** Saving the graph state to physical storage.

### 1.2 The Core Shard (The "OS/Distro")
The Shard is the specific configuration of nodes and edges that defines the user experience.
* **Responsibilities:**
    * Defining the initial set of Logic Nodes (e.g., a CLI or a GUI).
    * Setting up the default capability graph (who can see what).
    * Providing the "Standard Library" of modules.

## 2. The Core Paradigm: Graph as the Global State

In MISTGOS, the entire system state resides within a single, unified directed graph.

### 2.1 The Node (Atomic Unit)
A **Node** is an opaque container of either data or logic.
* **Logic Nodes:** Contain executable machine code (e.g., Rust-compiled WASM).
* **Data Nodes:** Contain raw byte-streams (blobs).
* **Port Nodes:** Represent hardware interfaces or I/O gateways.

### 2.2 The Edge (Capability & Relationship)
An **Edge** is a directed connection from one node to another. In the GMOS paradigm, an edge is a **Capability**.
* **Permissions:** Every edge carries a bitmask (Read, Write, Execute, Traverse).
* **Contextual Meaning:** Edges define relationships (e.g., "A *uses* B").

## 3. Security Model: Object-Capability (OCAP)

### 3.1 Invisibility and "The Mist"
The fundamental security rule: **If there is no path of edges from your current context to a node, that node is invisible.** This creates "The Mist"—resources are only revealed as capabilities are granted.

### 3.2 Capability Propagation
Capabilities are passed by creating new edges. Access is decentralized and intrinsic to the graph topology.

## 4. Execution Model: The PULSE Engine
The **PULSE** (Parallel Unit Logical Scheduling Engine) is the core execution loop.
1. **Activation:** Execution starts when a Logic Node is triggered.
2. **Context Isolation:** Each running process is limited to the subgraph reachable from its entry node.
3. **Messaging:** Inter-module communication happens via signals across edges.
