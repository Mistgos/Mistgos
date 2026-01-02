# MISTGOS

### "Everything is a file" was a great idea. In 1970.

I'm tired of folders. I'm tired of `PATH` variables, broken symlinks, and the mental overhead of hierarchical filesystems. MISTGOS is my attempt to escape the 50-year-old shadow of Unix and build something that actually makes sense for how we use data today.

This isn't a "Linux-killer." It's not a commercial product. It's a **GMOS** (Graph-based Modular Operating System)—a personal experiment in building a digital space that feels like a living organism rather than a filing cabinet.

## The Vision (In plain English)

Instead of a tree of folders, MISTGOS is a **Graph**.

Imagine a space where data and code just *exist*. You don't "open a file." You navigate to a **Node**. You don't "set permissions." You draw an **Edge**.

* **Security is Silence:** If there’s no connection (Edge) to a node, it literally doesn't exist for your current process. I call this **The Mist**. It’s not "Access Denied"; it’s "Nothing to see here."

* **Logic is Liquid:** All code is compiled to **WASM**. It’s sandboxed, portable, and runs wherever the graph takes it.

* **The Pulse:** The heart of the system is a scheduler that doesn't care about "apps." It cares about activations and signals moving through the graph.

## Why Rust?

Because I want to sleep at night. Building a kernel is hard enough without chasing null pointers and data races. Rust lets me focus on the architecture while the compiler yells at me when I try to do something stupid.

## Current Status: "It compiles (mostly)"

This is **Pre-Alpha**. Actually, it's more like a "Pre-Pre-Everything."

* [x] Manifesto written in a fit of late-night inspiration.
* [x] Architecture defined (the GMOS lineage).
* [x] EUPL 1.2 License chosen (Code for code, respect the lineage).
* [ ] PULSE Engine actually doing something useful (WIP).
* [ ] First bootable Shard.

**Warning:** This is a hobbyist project. It's experimental, opinionated, and highly likely to change completely tomorrow. I'm using AI to help me refine the code, but the vision and the frustration with modern OS architecture is 100% human.

## Community & Contributions

Currently, MISTGOS is a personal research project. **We are not accepting community fixes, pull requests, or contributions at this stage.** The system is in its infancy, and the architectural "Mist" is still being shaped. This policy might change in the far future as the core stabilizes, but for now, please do not open Pull Requests. You are, however, encouraged to fork the lineage and experiment with your own Shards.

## License

MISTGOS is licensed under the **EUPL 1.2**. Why? Because if you use my code to build something cool, I want that cool stuff to be shared back. It’s a "code-for-code" deal. I don't care what you do with your hardware — lock it down or open it up — but keep the software's bloodline clean.
