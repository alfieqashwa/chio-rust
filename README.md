# Chio Rust

## Learning Rust Again

The repository is a collection of files that summarize my learning journey with Rust. Each file corresponds to a specific topic or concept in Rust, and the naming convention follows a simple pattern: `00-vars`, `01-strings`, etc.

### Running the Code

```bash
cargo run -q --bin 05-structs
```

### The Stack & the Heap

| Stack                                        | Heap                                  |
| -------------------------------------------- | ------------------------------------- |
| Fixed size, known at compile time            | Dynamic size                          |
| Fast (direct access)                         | Slower (access via pointer)           |
| No manual allocation needed                  | Requires allocation via allocator     |
| Automatically cleaned up after function ends | Needs manual management via ownership |
| Suitable for small & fixed-size data         | Suitable for large/dynamic data       |

### Ownership Rules

1. Each value in Rust has an owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.

### The Rules of References

1. At any given time, you can have either one mutable reference or any number of immutable references.
2. References must always be valid.
