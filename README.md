# shm-ring

Lock-free Single Producer Single Consumer (SPSC) shared-memory ring buffer for Linux.

## Features

- POSIX shared memory (`shm_open`, `ftruncate`, `mmap`)
- Zero-copy communication
- Lock-free synchronization via atomics
- Power-of-two ring buffer
- Acquire/Release memory ordering

## Memory Layout

| SharedHeader |
|--------------|
| head (AtomicUsize) |
| tail (AtomicUsize) |
| capacity |
|--------------|
| ring buffer |

## Example

cargo run --example fork_demo
