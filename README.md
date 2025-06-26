# My Rust Async Book Tutorial

I created this interactive tutorial to assist myself in learning asynchronous programming in Rust. It's based on the [Rust Async Book](https://rust-lang.github.io/async-book/).

Each exercise is generated on-demand to keep the workspace clean.

## Overview

The theoretical concepts from the Rust Async Book and turned them into practical exercises. Instead of dumping everything at once, I've set up progressive exercises appear to go through each concept.

I'm a big believer in learning by doing, so everything is test-driven - your implementations get validated against real-world scenarios I've adapted from the async book.

## Features

- **Progressive Learning Path**: Exercises unlock sequentially, ensuring solid understanding before advancing
- **Test-Driven Validation**: Progress is determined by passing comprehensive test suites, not arbitrary markers
- **Integrated Learning**: Each exercise includes detailed documentation I've adapted from the async book
- **Clean Workspace**: Your working directory remains uncluttered with only active exercises visible
- **Persistent Progress**: Automatically tracks your advancement between sessions

## Prerequisites

- Rust 1.75 or later
- Basic familiarity with Rust syntax and ownership concepts
- A terminal and text editor

## Installation

Clone the repository and build the project:

```bash
git clone https://github.com/yourusername/async-book-tutorial.git
cd async-book-tutorial
cargo build --release
```

## Getting Started

Let's start your async journey with a single command:

```bash
cargo run -- next
```

This generates your first exercise in the `exercises/` directory. Open the file in your editor, read the comprehensive documentation at the top, and implement the required functionality.

Test your implementation:

```bash
cargo run -- check
```

When all tests pass, the next exercise automatically becomes available.

## Usage

### Core Commands

| Command | Description |
|---------|-------------|
| `cargo run -- next` | Start or continue with the next exercise |
| `cargo run -- check` | Validate your current implementation |
| `cargo run -- list` | View all exercises and your progress |
| `cargo run -- run <id>` | Jump to a specific unlocked exercise |
| `cargo run -- reset` | Clear all progress and start fresh |
| `cargo run -- help` | Display available commands |

### Convenience Script

For easier command execution, use the included wrapper script:

```bash
./async-book next
./async-book check
```

## Learning Path

I've organized the tutorial into five chapters with two exercises each:

1. **Getting Started** - Async/await fundamentals and performance benefits
2. **Under the Hood** - Future trait implementation and custom executors
3. **Advanced Async/Await** - Lifetime management and ownership in async contexts
4. **Concurrent Execution** - Working with multiple futures using join and select
5. **Streams and Iteration** - Asynchronous iteration patterns and stream processing

Each exercise builds upon previous concepts, creating a comprehensive understanding of Rust's async ecosystem.

## How It Works

### Exercise Structure

I've structured each exercise to follow a consistent pattern:

```rust
//! Comprehensive documentation explaining:
//! - Learning objectives
//! - Relevant async book concepts
//! - Implementation requirements
//! - Helpful examples

// Your implementation area - clean and distraction-free

#[cfg(test)]
mod tests {
    // Comprehensive test suite validating your solution
}
```

### Test-Driven Advancement

I use Rust's built-in testing framework to validate your implementations. The tests check:

- Correct async/await syntax usage
- Proper concurrent execution patterns
- Performance characteristics of async vs sync code
- Edge case handling and error management

Only when all tests pass will the next challenge unlock - that's how I know you're ready to move on.

### Progressive Generation

You'll start with a clean workspace, and exercises will appear as you progress:

```
exercises/
├── ch01_ex01.rs    # First exercise
├── ch01_ex02.rs    # Appears after completing 1.1
└── ...             # Further exercises unlock sequentially
```

I've found this approach helps maintain focus and prevents that overwhelming feeling of seeing too much content at once.

## Project Structure

```
.
├── Cargo.toml              # Single workspace configuration
├── src/
│   ├── main.rs            # CLI implementation
│   └── exercises/         # Exercise templates and generators
├── exercises/             # Generated exercise files (git-ignored)
├── progress.json          # Progress tracking (git-ignored)
└── async-book-source/     # Reference material from the async book
```

## Development

I've structured this as a single Cargo workspace to keep things simple - no complex nested projects. All dependencies are managed centrally, and everything works through standard Cargo commands.

The exercise templates live in `src/exercises/`, with each chapter module containing generators that produce the actual exercise files. I've found this separation keeps the source code clean while still allowing dynamic exercise generation.

## Troubleshooting

If you encounter issues:

1. Ensure you're using Rust 1.75 or later: `rustc --version`
2. Check that all tests compile: `cargo test --no-run`
3. View detailed test output: `cargo test --bin ch01_ex01 -- --nocapture`
4. Verify your current exercise: `cargo run -- list`

## Contributing

I'd love your contributions! Some areas I think could use improvement:

- Additional exercises covering more advanced topics
- Enhanced error messages and hints
- Performance benchmarks for async patterns
- Integration with popular async runtimes

If you add new exercises, please follow the pattern I've established with comprehensive documentation and thorough testing.

## License

This project is licensed under the MIT License. See the LICENSE file for details.

## Acknowledgments

This project builds on the excellent work of the Rust Async Book authors and the broader Rust async ecosystem. Big thanks to the Tokio and async-std teams for their foundational runtime implementations that make all this possible.
