# Contributing to struct-threads

First off, thank you for considering contributing to `struct-threads`! It's people like you that make open-source software such a great community.

## How Can I Contribute?

### Reporting Bugs & Requesting Features
- **Check existing issues:** Before opening a new issue, please check the [Issue Tracker](https://github.com/AsfhtgkDavid/struct-threads/issues) to see if someone else has already reported the problem or requested the feature.
- **Open a new issue:** If you can't find an existing issue, open a new one. Please provide as much detail as possible, including steps to reproduce the bug (a minimal reproducible example is best) or a clear use-case for the feature.

### Submitting Pull Requests

1. **Fork the repository** on GitHub.
2. **Clone your fork locally:**
```bash
git clone https://github.com/yourusername/struct-threads.git
cd struct-threads
```

3. **Create a new branch** for your feature or bugfix:
```bash
git checkout -b feature/my-awesome-feature
```

4. **Make your changes.** Be sure to add tests for any new functionality or bug fixes.
5. **Run the tests** to ensure your changes don't break existing functionality:
```bash
cargo test
```

6. **Format and Lint your code.** We follow standard Rust formatting and linting rules. Before committing, please ensure your code passes both:
```bash
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
```

7. **Commit your changes** with clear and descriptive commit messages.
8. **Push to your fork:**
```bash
git push origin feature/my-awesome-feature
```

9. **Open a Pull Request** against the `main` branch of the original repository. Describe your changes clearly in the PR description.

## Development Setup

To work on `struct-threads` locally, you will need to have [Rust and Cargo installed](https://rustup.rs/).

* Build the project: `cargo build`
* Run all tests: `cargo test`

Thank you for your contribution!