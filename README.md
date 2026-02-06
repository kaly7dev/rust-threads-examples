# Rust Threads Examples

Examples from the Medium post: "How to Use Your Full Power with Threads in Rust Programming".

## Setup
- `cargo build`
- Run an example: `cargo run --example basic_spawn`

## Examples
- basic_spawn: Simple thread spawn
- move_closure: Ownership with move
- scoped_threads: Scoped threads
- arc_immutable: Arc for immutable data
- mutex_counter: Mutex for mutable counter
- rwlock: RwLock for readers/writers
- mpsc_channel: std mpsc channels
- crossbeam_channel: Crossbeam channels
- atomics: Atomic operations
- rayon_pool: Rayon thread pool
- sharding: Sharding mutexes to reduce contention

For details, see the original post.