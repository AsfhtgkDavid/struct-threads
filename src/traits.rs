use std::sync::{Arc, Mutex};

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
}

impl<T: Runnable> Thread for T {
    fn start(self) -> std::thread::JoinHandle<Self::Output> {
        std::thread::spawn(move || self.run())
    }
}

/// An extension trait that provides a method to run multiple [`Runnable`]'s in parallel.
///
/// This trait is automatically implemented for any `Vec<T>` where `T` implements `Runnable`.
/// You do not need to implement this trait manually.
pub trait ParallelRun {
    type Output: Send + 'static;

    /// Spawns multiple threads to execute the `run` method of each task in parallel.
    ///
    /// The number of threads spawned will be equal to the number of available CPU cores.
    ///
    /// # Returns
    ///
    /// Returns a [`Vec<Self::Output>`] containing the results of each task, in the same order as the input vector.
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
    /// let results = tasks.par_run(); // Provided by the ParallelRun trait
    /// assert_eq!(results, vec![3, 7, 11]);
    /// ```
    fn par_run(self) -> Vec<Self::Output>;
}

impl<T: Runnable> ParallelRun for Vec<T> {
    type Output = T::Output;

    fn par_run(self) -> Vec<Self::Output> {
        let threads = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1);

        let queue = Arc::new(Mutex::new(self.into_iter()));
        let handles = (0..threads).map(|i| {
            let queue = Arc::clone(&queue);
            std::thread::spawn(move || {
                let mut results = Vec::new();
                loop {
                    let task = {
                        let mut q = queue.lock().unwrap();
                        q.next()
                    };
                    match task {
                        Some(task) => results.push(task.run()),
                        None => break,
                    }
                }
                (results, i)
            })
        });

        let mut results = handles
            .into_iter()
            .map(|h| h.join().unwrap())
            .collect::<Vec<_>>();
        results.sort_by_key(|&(_, i)| i);
        results.into_iter().flat_map(|(res, _)| res).collect()
    }
}
