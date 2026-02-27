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

## Features

- **Default**: Provides `Runnable`, `Thread`, and `ParallelRun` traits for standard thread-based execution
- **`tokio`**: Adds `AsyncRunnable`, `TokioTask`, and `TokioParallelRun` traits for async task execution with Tokio runtime

## Installation

Install using `cargo`:

```bash
cargo add struct-threads
```

For async support with Tokio:

```bash
cargo add struct-threads --features tokio
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

## Advanced Usage: Parallel Execution

You can run multiple tasks in parallel using the `ParallelRun` trait:

```rust
use struct_threads::{Runnable, ParallelRun};

struct ComputeTask(i32);

impl Runnable for ComputeTask {
    type Output = i32;

    fn run(self) -> Self::Output {
        // Perform some heavy computation
        self.0 * self.0
    }
}

fn main() {
    let tasks = (0..100)
        .map(ComputeTask)
        .collect::<Vec<_>>();
    
    // Runs tasks in parallel across available CPU cores
    let results = tasks.par_run().unwrap();
    
    println!("Computed {} results", results.len());
}
```

## Advanced Usage: Custom Thread Configuration

Use `start_with_builder` to customize thread properties:

```rust
use std::thread::Builder;
use struct_threads::{Runnable, Thread};

struct MyTask(i32);

impl Runnable for MyTask {
    type Output = i32;

    fn run(self) -> Self::Output {
        self.0 * 2
    }
}

fn main() {
    let task = MyTask(21);
    
    let builder = Builder::new()
        .name("my-custom-thread".to_string())
        .stack_size(4 * 1024 * 1024); // 4 MB stack
    
    let handle = task.start_with_builder(builder);
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

## Async Support (Tokio)

Enable the `tokio` feature to use async tasks:

```toml
[dependencies]
struct-threads = { version = "1.0", features = ["tokio"] }
tokio = { version = "1", features = ["rt", "macros"] }
```

### Basic Async Usage

```rust
use struct_threads::{AsyncRunnable, TokioTask};

struct AsyncTask(i32);

impl AsyncRunnable for AsyncTask {
    type Output = i32;

    fn run(self) -> impl std::future::Future<Output = Self::Output> + Send {
        async move {
            // Perform async work
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            self.0 * 2
        }
    }
}

#[tokio::main]
async fn main() {
    let task = AsyncTask(21);
    
    // The .async_start() method is provided by the TokioTask trait
    let handle = task.async_start();
    
    // Await the result
    let result = handle.await.unwrap();
    
    println!("Result: {}", result);
}
```

### Parallel Async Execution

```rust
use struct_threads::{AsyncRunnable, TokioParallelRun};

struct AsyncComputeTask(i32);

impl AsyncRunnable for AsyncComputeTask {
    type Output = i32;

    fn run(self) -> impl std::future::Future<Output = Self::Output> + Send {
        async move {
            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
            self.0 * self.0
        }
    }
}

#[tokio::main]
async fn main() {
    let tasks = (0..100)
        .map(AsyncComputeTask)
        .collect::<Vec<_>>();
    
    // Runs async tasks in parallel
    let results = tasks.async_par_run().await.unwrap();
    
    println!("Computed {} results", results.len());
}

```

## Contributing

Contributions are welcome! To get started:

1. Clone the repository: `git clone https://github.com/AsfhtgkDavid/struct-threads.git`
2. Run tests to ensure everything works locally: `cargo test`
3. Submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](https://www.google.com/search?q=LICENSE) file for details.