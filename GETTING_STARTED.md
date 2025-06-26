# Getting Started

## Quick Start

```bash
# Generate your first exercise
cargo run -- next

# Open exercises/ch01_ex01.rs in your editor

# Test your solution
cargo run -- check
```

## Command Reference

| Command | Shorthand | Purpose |
|---------|-----------|---------|
| `cargo run -- next` | `./async-book next` | Continue learning |
| `cargo run -- check` | `./async-book check` | Test current exercise |
| `cargo run -- list` | `./async-book list` | View progress |
| `cargo run -- run 1.2` | `./async-book run 1.2` | Jump to exercise |
| `cargo run -- reset` | `./async-book reset` | Start over |

## Exercise Workflow

1. **Read** - Each exercise starts with comprehensive documentation
2. **Implement** - Write code in the designated areas
3. **Test** - Run `check` to validate your solution
4. **Progress** - Pass all tests to unlock the next exercise

## Tips

- Read test names carefully - they describe expected behavior
- Use `cargo test --bin ch01_ex01 -- --nocapture` for detailed output
- Focus on making one test pass at a time
- The compiler's error messages are your friend

## Common Issues

**"No such file or directory"**  
Run `cargo run -- next` first to generate the exercise.

**"Tests are failing"**  
This is normal! Read the test output to understand what to implement.

**"Cannot run exercise X.X"**  
Complete previous exercises first. Check progress with `cargo run -- list`.

## Next Steps

Ready to begin? Start with:

```bash
cargo run -- next
```

For comprehensive documentation, see the [README](README.md).