# SigmaOS-allocators Component Wiki

## Overview
SigmaOS-allocators provides custom allocator implementations and memory allocator support for the SigmaOS operating system. It demonstrates the allocator subsystem and custom memory management within the SigmaOS ecosystem, following the principles of clear abstraction and deterministic allocator initialization.

## Principles
- Single Responsibility
- Allocator Abstraction
- Fail-Fast on Allocator Initialization Errors
- Deterministic Allocator Setup
- Clear Abstraction Over Allocator APIs
- Composition over Inheritance for allocator extensions

## Architecture
SigmaOS-allocators follows an allocator pattern:
- **Allocator Core**: Provides custom memory allocator implementations, memory tracking, and allocator registration
- **Memory Tracking**: Supports allocation tracking, memory profiling, and allocator metrics
- **Allocator Registration**: Supports allocator registration, allocator switching, and allocator metrics
- **Error Handling**: Fail-fast on invalid allocator configurations or unsupported allocator types

## API
The component provides the following public APIs:
- `allocator::register(name, allocator)` - Register a custom allocator with given name
- `allocator::use_allocator(name)` - Use a registered allocator by name
- `allocator::status::trackers()` - Get allocator status and active trackers
- `allocator::metrics::collect()` - Collect allocator metrics and memory statistics
- Error types for allocator failure scenarios or unsupported allocator types

## Test Structure (Arrange-Act-ASSERT)
All tests follow the AAA pattern:
1. **Arrange**: Set up test environment with specific allocator configuration
2. **Act**: Execute the code under test (allocator registration, allocator usage)
3. **Assert**: Verify the expected allocator results

Example:
```rust
#[test]
fn test_allocator_register_aaa() {
    // Arrange
    let name = "jemalloc";
    let allocator = CustomAllocator::new();
    
    // Act
    let result = allocator::register(name, allocator);
    
    // Assert
    assert!(result.is_ok());
    let registered = result.unwrap();
    assert!(registered.is_valid());
}
```

## Development
- Follows Rust best practices with `#![forbid(unsafe_code)]`
- Allocator initialization is best-effort; gracefully degrades on unsupported configurations
- Comprehensive error types for all allocator failure scenarios
- Unit tests provided for allocator registration and allocator usage
- Integration tests for full allocator system with actual custom memory management

## TODO
- Add support for additional allocators (jemalloc, mimalloc, tcmalloc)
- Expand memory tracking capabilities (allocation traces, memory footprints, allocation history)
- Add allocator features (fallback allocators, allocator switching, allocator metrics)
- Create integration tests with actual custom allocator implementations
- Implement allocator persistence and allocator state management