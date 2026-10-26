//! SigmaOS Allocators
//! Memory allocators (slab, buddy)

pub mod slab;
pub mod buddy;

pub use slab::SlabAllocator;
