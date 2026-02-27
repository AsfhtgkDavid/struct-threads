/// A trait for defining a task that can be executed, typically in a separate thread.
///
/// Types implementing `Runnable` must be `Send` and `'static` to ensure they
/// can be safely transferred across thread boundaries. By encapsulating state
/// within a struct that implements this trait, you can easily manage complex
/// thread initialization.
///
/// # Examples
///
/// ```rust
/// use struct_threads::Runnable;
///
/// struct GreetingTask {
///     name: String,
/// }
///
/// impl Runnable for GreetingTask {
///     type Output = String;
///
///     fn run(self) -> Self::Output {
///         format!("Hello, {}!", self.name)
///     }
/// }
/// ```
pub trait Runnable: Send + 'static {
    /// The type of value returned when the task completes.
    type Output: Send + 'static;

    /// Executes the task logic.
    ///
    /// This method consumes the task (`self`), meaning the state cannot be
    /// reused after the thread has finished executing.
    fn run(self) -> Self::Output;
}

/// An extension trait that provides a method to spawn a thread for a [`Runnable`] task.
///
/// This trait is automatically implemented for any type that implements `Runnable`.
/// You do not need to implement this trait manually.
pub trait Thread: Runnable {
    /// Spawns a new standard library thread to execute the `run` method.
    ///
    /// This acts as a zero-cost abstraction over [`std::thread::spawn`].
    ///
    /// # Returns
    ///
    /// Returns a [`std::thread::JoinHandle`] that can be used to wait for the thread
    /// to finish and extract its `Output`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use struct_threads::{Runnable, Thread};
    ///
    /// struct MathTask(i32, i32);
    ///
    /// impl Runnable for MathTask {
    ///     type Output = i32;
    ///     fn run(self) -> Self::Output {
    ///         self.0 + self.1
    ///     }
    /// }
    ///
    /// let task = MathTask(5, 7);
    /// let handle = task.start(); // Provided by the Thread trait
    ///
    /// assert_eq!(handle.join().unwrap(), 12);
    /// ```
    fn start(self) -> std::thread::JoinHandle<Self::Output>;

    /// Spawns a new thread using a custom [`std::thread::Builder`] to execute the `run` method.
    ///
    /// This allows you to configure thread properties such as name, stack size, or other
    /// platform-specific options before spawning.
    ///
    /// # Arguments
    ///
    /// * `builder` - A [`std::thread::Builder`] configured with the desired thread properties
    ///
    /// # Returns
    ///
    /// Returns a [`std::thread::JoinHandle`] that can be used to wait for the thread
    /// to finish and extract its `Output`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::thread::Builder;
    /// use struct_threads::{Runnable, Thread};
    ///
    /// struct MathTask(i32, i32);
    ///
    /// impl Runnable for MathTask {
    ///     type Output = i32;
    ///     fn run(self) -> Self::Output {
    ///         self.0 + self.1
    ///     }
    /// }
    ///
    /// let task = MathTask(5, 7);
    /// let builder = Builder::new()
    ///     .name("math-thread".to_string())
    ///     .stack_size(4 * 1024 * 1024); // 4 MB stack
    ///
    /// let handle = task.start_with_builder(builder);
    /// assert_eq!(handle.join().unwrap(), 12);
    /// ```
    fn start_with_builder(
        self,
        builder: std::thread::Builder,
    ) -> std::thread::JoinHandle<Self::Output>;
}

impl<T: Runnable> Thread for T {
    fn start(self) -> std::thread::JoinHandle<Self::Output> {
        std::thread::spawn(move || self.run())
    }
    fn start_with_builder(
        self,
        builder: std::thread::Builder,
    ) -> std::thread::JoinHandle<Self::Output> {
        builder.spawn(move || self.run()).unwrap()
    }
}

/// An extension trait that provides a method to run multiple [`Runnable`]'s in parallel.
///
/// This trait is automatically implemented for any `Vec<T>` where `T` implements `Runnable`.
/// You do not need to implement this trait manually.
///
/// The parallel execution is optimized to use the number of available CPU cores,
/// dividing the tasks into chunks and processing them concurrently.
pub trait ParallelRun {
    type Output: Send + 'static;

    /// Spawns multiple threads to execute the `run` method of each task in parallel.
    ///
    /// The number of threads spawned is determined by the number of available CPU cores,
    /// with tasks divided evenly among them. Each thread processes a chunk of tasks
    /// sequentially, while chunks are processed in parallel.
    ///
    /// # Returns
    ///
    /// Returns a [`std::thread::Result<Vec<Self::Output>>`] containing the results of each task,
    /// in the same order as the input vector. Returns an error if any thread panics.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use struct_threads::{Runnable, ParallelRun};
    ///
    /// struct MathTask(i32, i32);
    ///
    /// impl Runnable for MathTask {
    ///    type Output = i32;
    ///
    ///   fn run(self) -> Self::Output {
    ///       self.0 + self.1
    ///   }
    /// }
    ///
    /// let tasks = vec![MathTask(1, 2), MathTask(3, 4), MathTask(5, 6)];
    ///
    /// let results = tasks
    ///     .par_run()
    ///     .unwrap(); // Provided by the ParallelRun trait
    /// assert_eq!(results, vec![3, 7, 11]);
    /// ```
    fn par_run(self) -> std::thread::Result<Vec<Self::Output>>;
}

impl<T: Runnable> ParallelRun for Vec<T> {
    type Output = T::Output;

    fn par_run(self) -> std::thread::Result<Vec<Self::Output>> {
        let threads = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1)
            .min(self.len());

        if threads == 0 {
            return Ok(Vec::new());
        }

        let chunk_size = self.len().div_ceil(threads);

        let mut iter = self.into_iter();
        let mut handles = Vec::with_capacity(threads);

        for _ in 0..threads {
            let chunk = iter.by_ref().take(chunk_size).collect::<Vec<_>>();
            let handle =
                std::thread::spawn(move || chunk.into_iter().map(|t| t.run()).collect::<Vec<_>>());
            handles.push(handle);
        }

        let results = handles
            .into_iter()
            .map(|h| h.join())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(results.into_iter().flatten().collect())
    }
}

/// A trait for defining an async task that can be executed, typically in a Tokio runtime.
///
/// Types implementing `AsyncRunnable` must be `Send` and `'static` to ensure they
/// can be safely transferred across async task boundaries. This trait is designed for
/// async operations and works seamlessly with the Tokio runtime when the `tokio` feature is enabled.
///
/// # Examples
///
/// ```rust
/// use struct_threads::AsyncRunnable;
///
/// struct AsyncGreetingTask {
///     name: String,
/// }
///
/// impl AsyncRunnable for AsyncGreetingTask {
///     type Output = String;
///
///     fn run(self) -> impl std::future::Future<Output = Self::Output> + Send {
///         async move {
///             // Simulate async work
///             tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
///             format!("Hello, {}!", self.name)
///         }
///     }
/// }
/// ```
pub trait AsyncRunnable: Send + 'static {
    /// The type of value returned when the async task completes.
    type Output: Send + 'static;

    /// Executes the async task logic.
    ///
    /// This method consumes the task (`self`) and returns a future that must be
    /// awaited to get the result.
    fn run(self) -> impl std::future::Future<Output = Self::Output> + Send;
}

/// An extension trait that provides a method to spawn a Tokio task for an [`AsyncRunnable`] task.
///
/// This trait is automatically implemented for any type that implements `AsyncRunnable`
/// when the `tokio` feature is enabled. You do not need to implement this trait manually.
///
/// # Examples
///
/// ```rust
/// use struct_threads::{AsyncRunnable, TokioTask};
///
/// struct AsyncMathTask(i32, i32);
///
/// impl AsyncRunnable for AsyncMathTask {
///     type Output = i32;
///
///     fn run(self) -> impl std::future::Future<Output = Self::Output> + Send {
///         async move {
///             tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
///             self.0 + self.1
///         }
///     }
/// }
///
/// #[tokio::main]
/// async fn main() {
///     let task = AsyncMathTask(5, 7);
///     let handle = task.async_start(); // Provided by the TokioTask trait
///
///     assert_eq!(handle.await.unwrap(), 12);
/// }
/// ```
#[cfg(feature = "tokio")]
pub trait TokioTask: AsyncRunnable {
    /// Spawns a new Tokio task to execute the `run` method.
    ///
    /// This acts as a zero-cost abstraction over [`tokio::task::spawn`].
    ///
    /// # Returns
    ///
    /// Returns a [`tokio::task::JoinHandle`] that can be awaited to get the task's output.
    fn async_start(self) -> tokio::task::JoinHandle<Self::Output>;
}

#[cfg(feature = "tokio")]
impl<T: AsyncRunnable> TokioTask for T {
    fn async_start(self) -> tokio::task::JoinHandle<Self::Output> {
        tokio::task::spawn(async move { self.run().await })
    }
}

/// An extension trait that provides a method to run multiple [`AsyncRunnable`]'s in parallel using Tokio.
///
/// This trait is automatically implemented for any `Vec<T>` where `T` implements `AsyncRunnable`
/// when the `tokio` feature is enabled. You do not need to implement this trait manually.
///
/// # Examples
///
/// ```rust
/// use struct_threads::{AsyncRunnable, TokioParallelRun};
///
/// struct AsyncMathTask(i32, i32);
///
/// impl AsyncRunnable for AsyncMathTask {
///     type Output = i32;
///
///     fn run(self) -> impl std::future::Future<Output = Self::Output> + Send {
///         async move {
///             tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
///             self.0 + self.1
///         }
///     }
/// }
///
/// #[tokio::main]
/// async fn main() {
///     let tasks = vec![
///         AsyncMathTask(1, 2),
///         AsyncMathTask(3, 4),
///         AsyncMathTask(5, 6)
///     ];
///
///     let results = tasks.async_par_run().await.unwrap();
///     assert_eq!(results, vec![3, 7, 11]);
/// }
/// ```
#[cfg(feature = "tokio")]
pub trait TokioParallelRun {
    /// The type of output produced by each task.
    type Output: Send + 'static;

    /// Spawns multiple Tokio tasks to execute the `run` method of each task in parallel.
    ///
    /// All tasks are spawned concurrently and their results are collected in the same order
    /// as the input vector.
    ///
    /// # Returns
    ///
    /// Returns a future that resolves to `Result<Vec<Self::Output>, tokio::task::JoinError>`
    /// containing the results of each task, or a join error if any task panics.
    fn async_par_run(self) -> impl std::future::Future<Output = Result<Vec<Self::Output>, tokio::task::JoinError>> + Send;
}

#[cfg(feature = "tokio")]
impl <T: AsyncRunnable> TokioParallelRun for Vec<T> {
    type Output = T::Output;

    fn async_par_run(self) -> impl std::future::Future<Output = Result<Vec<Self::Output>, tokio::task::JoinError>> + Send {
        async move {
            let handles: Vec<_> = self.into_iter().map(|t| tokio::task::spawn(async { t.run().await })).collect();
            let mut result = Vec::with_capacity(handles.len());

            for handle in handles {
                result.push(handle.await?);
            }
            Ok(result)
        }
    }
}