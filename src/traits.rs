/// A trait for defining a task that can be executed, typically in a separate thread.
///
/// Types implementing `Runnable` must be `Send` and `'static` to ensure they
/// can be safely transferred across thread boundaries.
pub trait Runnable: Send + 'static {
    /// The type of value returned when the task completes.
    type Output: Send + 'static;

    /// Executes the task logic.
    fn run(self) -> Self::Output;
}

/// An extension trait that provides a method to spawn a thread for a `Runnable` task.
pub trait Thread: Runnable {
    /// Spawns a new standard library thread to execute the `run` method.
    ///
    /// Returns a `JoinHandle` that can be used to wait for the thread to finish
    /// and extract its `Output`.
    fn start(self) -> std::thread::JoinHandle<Self::Output>;
}

impl<T: Runnable> Thread for T {
    fn start(self) -> std::thread::JoinHandle<Self::Output> {
        std::thread::spawn(move || self.run())
    }
}
