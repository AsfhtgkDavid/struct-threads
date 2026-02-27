# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.1.0] - 2026-02-27

### Added
- **Async Support**: New `tokio` feature flag for async task execution
- **AsyncRunnable trait**: Define async tasks that can be executed with Tokio runtime
- **TokioTask trait**: Extension trait providing `async_start()` method for spawning async tasks
- **TokioParallelRun trait**: Run multiple async tasks in parallel with `async_par_run()` method
- **Custom Thread Builder**: `start_with_builder()` method for customizing thread properties (name, stack size, etc.)
- **Parallel Execution**: `ParallelRun` trait for running multiple tasks in parallel across CPU cores
- Comprehensive documentation for all features
- Examples for async usage, parallel execution, and custom thread configuration

### Changed
- Enhanced documentation with detailed examples for all traits
- Improved README with feature descriptions and usage examples

## [1.0.0] - Initial Release

### Added
- **Runnable trait**: Define tasks as structs implementing a simple trait
- **Thread trait**: Extension trait providing `start()` method for spawning threads
- Basic documentation and examples

