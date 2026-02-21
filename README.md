# struct-threads

[![Crates.io](https://img.shields.io/crates/v/struct-threads.svg)](https://crates.io/crates/struct-threads)
[![Docs.rs](https://docs.rs/struct-threads/badge.svg)](https://docs.rs/struct-threads)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A simple Rust library that provides a clean, object-oriented way to structure thread logic by using `struct`s implementing a `Runnable` trait.

## Motivation

While `std::thread::spawn` is powerful, complex thread logic often requires passing multiple variables, channels, or shared state into a closure. This can make code difficult to read and maintain. 

`struct-threads` solves this by encapsulating thread state and logic within a struct. This approach:
- **Improves readability** by separating state initialization from execution.
- **Simplifies testing** by allowing you to instantiate thread state without immediately spawning it.
- **Provides a clear contract** for the thread's return value via associated types.

## Installation

Install using `cargo`:

```bash
cargo add struct-threads

```

## Basic Usage

Define your task state in a struct, implement `Runnable`, and call `.start()`.

```rust
use struct_threads::{Runnable, Thread};

struct MyTask {
    data: i32,
}

impl Runnable for MyTask {
    type Output = i32;

    fn run(self) -> Self::Output {
        println!("Running task in a separate thread...");
        std::thread::sleep(std::time::Duration::from_millis(100));
        self.data * 2
    }
}

fn main() {
    let task = MyTask { data: 21 };
    
    // The .start() method is provided by the Thread trait
    let handle = task.start();
    
    // Wait for the thread to finish and get the result
    let result = handle.join().unwrap();
    
    println!("Result: {}", result);
}

```

## Advanced Usage: Channels

This pattern truly shines when your thread needs to communicate with the main thread or hold more complex state, like channels.

```rust
use std::sync::mpsc;
use struct_threads::{Runnable, Thread};

struct WorkerTask {
    id: usize,
    sender: mpsc::Sender<String>,
}

impl Runnable for WorkerTask {
    type Output = ();

    fn run(self) -> Self::Output {
        // Perform work...
        let msg = format!("Worker {} has successfully finished its job!", self.id);
        self.sender.send(msg).unwrap();
    }
}

fn main() {
    let (tx, rx) = mpsc::channel();
    
    let worker = WorkerTask { id: 1, sender: tx };
    worker.start(); // Spawns the thread
    
    // Wait for the message from the worker
    let result = rx.recv().unwrap();
    println!("Received: {}", result);
}

```

## Contributing

Contributions are welcome! To get started:

1. Clone the repository: `git clone https://github.com/AsfhtgkDavid/struct-threads.git`
2. Run tests to ensure everything works locally: `cargo test`
3. Submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](https://www.google.com/search?q=LICENSE) file for details.