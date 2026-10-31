# SigmaOS Allocators

Low-level Rust memory allocation primitives for SigmaOS, currently comprising a buddy frame allocator and slab allocation/cache code.

## Components

- `buddy`: page-frame bookkeeping and order-based allocation, with 4 KiB pages and orders through `MAX_ORDER`.
- `slab`: fixed-size object allocation from caller-provided memory regions.
- `SlabAllocator`: the crate-root re-export from `slab`.

These APIs operate on raw memory and include unsafe initialization, allocation, and freeing operations. The caller must provide valid, correctly aligned, exclusively managed regions and preserve allocator invariants. This crate is a kernel component, not a general-purpose application allocator; review the implementation and safety contracts before use.

## Build

The manifest uses a sibling path dependency on [`SigmaOS-kernel`](../SigmaOS-kernel). From the SigmaOS checkout:

```sh
cargo check --manifest-path SigmaOS-allocators/Cargo.toml --lib
```

## License

GPL-3.0-or-later. See [`Cargo.toml`](Cargo.toml).
