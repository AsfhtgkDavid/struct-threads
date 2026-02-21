pub mod traits;

pub use traits::{Runnable, Thread};

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
}
