## Shared-Memory-IPC-Ring-Buffer
**Shared-Memory-IPC-Ring-Buffer** is an open-source software project by **sufyanism** on GitHub. It provides a high-performance implementation of interprocess communication (IPC) using shared memory and a ring buffer structure. The project aims to facilitate fast, lock-free data exchange between processes in systems where low latency and high throughput are critical.

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


![Image](https://www.researchgate.net/publication/258668692/figure/fig2/AS%3A682281324793859%401539679952684/nterprocess-communication-IPC-is-implemented-as-a-ring-buffer-in-shared-memory-space.ppm)

![Image](https://www.researchgate.net/publication/335755427/figure/fig2/AS%3A888401746202627%401588822890285/The-shared-ring-buffers-FIFO-between-NIC-and-the-device-driver.png)

![Image](https://media.licdn.com/dms/image/v2/C4D12AQHcHm42WvmF8w/article-cover_image-shrink_600_2000/article-cover_image-shrink_600_2000/0/1619262884390?e=2147483647\&t=qHqdxCrIbZDyiZDvgrdmIY2-1qh3IA0XK2xlMu_pyjE\&v=beta)

![Image](https://www.tutorialspoint.com/inter_process_communication/images/shared_memory.jpg)

![Image](https://sujith-eag.in/os/3_12_CommunicationsModels.jpg)


### Key facts
* **Developer:** sufyanism
* **Platform:** Cross-platform (C/C++)
* **Core concept:** Shared memory + lock-free ring buffer
* **Use case:** High-speed IPC between processes
* **License:** MIT License

### Design and purpose
The project implements a ring buffer data structure allocated in shared memory, allowing multiple processes to read and write concurrently without heavy synchronization primitives. By minimizing context switches and kernel involvement, it achieves significant performance gains over socket or pipe-based IPC methods.

### Implementation details
The ring buffer uses atomic operations for synchronization, supporting non-blocking read/write operations. Each process maps the same shared memory segment, maintaining local cursors for data production and consumption. The design ensures consistent visibility of data across processes while avoiding race conditions through memory barriers and atomic counters.

### Typical applications
Shared-Memory-IPC-Ring-Buffer is suited for high-frequency data exchange, such as in trading systems, telemetry pipelines, and real-time analytics. Developers adopt it to reduce latency where processes must communicate large volumes of small messages efficiently.

