//! A simple library providing a clean, object-oriented way to structure thread logic.
//!
//! `struct-threads` allows you to define your task state in a `struct`, implement the
//! [`Runnable`] trait, and seamlessly spawn it using the [`Thread`] extension trait.
//!
//! # Features
//!
//! - **Default**: Provides [`Runnable`], [`Thread`], and [`ParallelRun`] traits for standard thread-based execution
//! - **`tokio`**: Adds [`AsyncRunnable`], [`TokioTask`], and [`TokioParallelRun`] traits for async task execution with Tokio runtime
//!
//! # Basic Example
//!
//! ```rust
//! use struct_threads::{Runnable, Thread};
//!
//! struct MyTask(i32);
//!
//! impl Runnable for MyTask {
//!     type Output = i32;
//!
//!     fn run(self) -> Self::Output {
//!         self.0 * 2
//!     }
//! }
//!
//! let handle = MyTask(21).start();
//! assert_eq!(handle.join().unwrap(), 42);
//! ```
//!
//! # Parallel Execution
//!
//! With `struct-threads`, you can also run multiple tasks in parallel using the [`ParallelRun`] extension trait.
//!
//! ```rust
//! use struct_threads::{Runnable, ParallelRun};
//!
//! struct MyTask(i32);
//!
//! impl Runnable for MyTask {
//!     type Output = i32;
//!
//!    fn run(self) -> Self::Output {
//!        self.0 * 2
//!    }
//! }
//!
//! let results = (0..10)
//!     .map(MyTask)
//!     .collect::<Vec<_>>()
//!     .par_run()
//!     .unwrap();
//!
//!  assert_eq!(results, (0..10).map(|x| x * 2).collect::<Vec<_>>());
//! ```
//!
//! # Custom Thread Configuration
//!
//! You can customize thread properties using the [`Thread::start_with_builder`] method:
//!
//! ```rust
//! use std::thread::Builder;
//! use struct_threads::{Runnable, Thread};
//!
//! struct MyTask(i32);
//!
//! impl Runnable for MyTask {
//!     type Output = i32;
//!     fn run(self) -> Self::Output { self.0 * 2 }
//! }
//!
//! let builder = Builder::new()
//!     .name("custom-thread".to_string())
//!     .stack_size(4 * 1024 * 1024);
//!
//! let handle = MyTask(21).start_with_builder(builder);
//! assert_eq!(handle.join().unwrap(), 42);
//! ```
//!
//! # Async Support (requires `tokio` feature)
//!
//! Enable async support by adding the `tokio` feature:
//!
//! ```toml
//! [dependencies]
//! struct-threads = { version = "1.0", features = ["tokio"] }
//! ```
//!
//! Then use [`AsyncRunnable`] and [`TokioTask`]:
//!
//! ```rust,ignore
//! use struct_threads::{AsyncRunnable, TokioTask};
//!
//! struct AsyncTask(i32);
//!
//! impl AsyncRunnable for AsyncTask {
//!     type Output = i32;
//!
//!     fn run(self) -> impl std::future::Future<Output = Self::Output> + Send {
//!         async move {
//!             tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
//!             self.0 * 2
//!         }
//!     }
//! }
//!
//! #[tokio::main]
//! async fn main() {
//!     let handle = AsyncTask(21).async_start();
//!     assert_eq!(handle.await.unwrap(), 42);
//! }
//! ```
#![cfg_attr(docsrs, feature(doc_cfg))]
pub mod traits;

pub use traits::{AsyncRunnable, ParallelRun, Runnable, Thread};

#[cfg(feature = "tokio")]
pub use traits::{TokioParallelRun, TokioTask};

#[cfg(test)]
mod tests {
    use super::*;

    struct TestTask(i32);
    impl Runnable for TestTask {
        type Output = i32;

        fn run(self) -> Self::Output {
            self.0 * 2
        }
    }

    #[test]
    fn it_works() {
        let task = TestTask(10);
        let handle = task.start();
        assert_eq!(handle.join().unwrap(), 20);
    }

    #[test]
    fn par_test() {
        let tasks = (0..1_000_000).map(TestTask).collect::<Vec<_>>();

        let results = tasks.par_run().unwrap();
        assert_eq!(results, (0..1_000_000).map(|x| x * 2).collect::<Vec<_>>());
    }

    #[test]
    fn par_test_empty() {
        let results = Vec::<TestTask>::new().par_run().unwrap();
        assert!(results.is_empty());
    }

    #[test]
    fn test_builder() {
        let task = TestTask(10);
        let builder = std::thread::Builder::new().name("custom_thread".to_string());
        let handle = task.start_with_builder(builder);
        assert_eq!(handle.join().unwrap(), 20);

        struct BuilderTask;

        impl Runnable for BuilderTask {
            type Output = String;

            fn run(self) -> Self::Output {
                std::thread::current().name().unwrap().to_string()
            }
        }

        let builder = std::thread::Builder::new().name("custom_thread".to_string());
        let handle = BuilderTask.start_with_builder(builder);
        assert_eq!(handle.join().unwrap(), "custom_thread".to_string());
    }

    #[cfg(feature = "tokio")]
    mod async_tests {
        use super::*;

        struct AsyncTestTask(i32);

        impl AsyncRunnable for AsyncTestTask {
            type Output = i32;

            fn run(self) -> impl std::future::Future<Output = Self::Output> + Send {
                async move {
                    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
                    self.0 * 2
                }
            }
        }

        #[tokio::test]
        async fn test_async_task() {
            let task = AsyncTestTask(21);
            let handle = task.async_start();
            assert_eq!(handle.await.unwrap(), 42);
        }

        #[tokio::test]
        async fn test_async_parallel_run() {
            let tasks = (0..10).map(AsyncTestTask).collect::<Vec<_>>();
            let results = tasks.async_par_run().await.unwrap();
            assert_eq!(results, (0..10).map(|x| x * 2).collect::<Vec<_>>());
        }

        #[tokio::test]
        async fn test_async_parallel_run_empty() {
            let tasks = Vec::<AsyncTestTask>::new();
            let results = tasks.async_par_run().await.unwrap();
            assert!(results.is_empty());
        }

        #[tokio::test]
        async fn test_async_parallel_run_large() {
            let tasks = (0..1_000).map(AsyncTestTask).collect::<Vec<_>>();
            let results = tasks.async_par_run().await.unwrap();
            assert_eq!(results, (0..1_000).map(|x| x * 2).collect::<Vec<_>>());
        }

        struct AsyncComplexTask {
            value: i32,
            multiplier: i32,
        }

        impl AsyncRunnable for AsyncComplexTask {
            type Output = i32;

            fn run(self) -> impl std::future::Future<Output = Self::Output> + Send {
                async move {
                    tokio::time::sleep(tokio::time::Duration::from_millis(5)).await;
                    self.value * self.multiplier
                }
            }
        }

        #[tokio::test]
        async fn test_async_complex_task() {
            let task = AsyncComplexTask {
                value: 7,
                multiplier: 6,
            };
            let handle = task.async_start();
            assert_eq!(handle.await.unwrap(), 42);
        }

        #[tokio::test]
        async fn test_async_multiple_awaits() {
            let task1 = AsyncTestTask(10);
            let task2 = AsyncTestTask(20);
            let task3 = AsyncTestTask(30);

            let handle1 = task1.async_start();
            let handle2 = task2.async_start();
            let handle3 = task3.async_start();

            let result1 = handle1.await.unwrap();
            let result2 = handle2.await.unwrap();
            let result3 = handle3.await.unwrap();

            assert_eq!(result1, 20);
            assert_eq!(result2, 40);
            assert_eq!(result3, 60);
        }

        struct AsyncStringTask(String);

        impl AsyncRunnable for AsyncStringTask {
            type Output = String;

            fn run(self) -> impl std::future::Future<Output = Self::Output> + Send {
                async move {
                    tokio::time::sleep(tokio::time::Duration::from_millis(5)).await;
                    format!("Hello, {}!", self.0)
                }
            }
        }

        #[tokio::test]
        async fn test_async_string_output() {
            let task = AsyncStringTask("World".to_string());
            let handle = task.async_start();
            assert_eq!(handle.await.unwrap(), "Hello, World!");
        }

        #[tokio::test]
        async fn test_async_parallel_run_strings() {
            let tasks = vec![
                AsyncStringTask("Alice".to_string()),
                AsyncStringTask("Bob".to_string()),
                AsyncStringTask("Charlie".to_string()),
            ];
            let results = tasks.async_par_run().await.unwrap();
            assert_eq!(
                results,
                vec![
                    "Hello, Alice!".to_string(),
                    "Hello, Bob!".to_string(),
                    "Hello, Charlie!".to_string()
                ]
            );
        }
    }
}
