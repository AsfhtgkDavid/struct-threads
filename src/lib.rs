//! A simple library providing a clean, object-oriented way to structure thread logic.
//!
//! `struct-threads` allows you to define your task state in a `struct`, implement the
//! [`Runnable`] trait, and seamlessly spawn it using the [`Thread`] extension trait.
//!
//! # Example
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
//! With `struct-threads`, you can also run multiple tasks in parallel using the [`ParallelRun`] extension trait.
//!
//! # Example
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

pub mod traits;

pub use traits::{ParallelRun, Runnable, Thread};

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
}
