# Project Context for AI Assistants

This document provides essential context for AI assistants working with my Rust Async Book tutorial project.

## Project Overview

My interactive tutorial that teaches Rust asynchronous programming through hands-on exercises. I've transformed theoretical concepts from the Rust Async Book into practical, test-driven exercises that unlock progressively as you demonstrate mastery.

## Architecture

### Core Design Principles

1. **Single Cargo Workspace** - All functionality consolidated into one project
2. **Progressive Generation** - Exercises appear only when the learner is ready
3. **Test-Driven Validation** - Progress determined by passing comprehensive test suites
4. **Clean Implementation Areas** - Educational content separated from code areas

### Key Components

**Main Application** (`src/main.rs`)
- Command-line interface for exercise management
- Progress tracking with JSON persistence
- Dynamic exercise file generation
- Test execution and validation

**Exercise System** (`src/exercises/`)
- Template modules for each chapter
- Generators that produce exercise files on demand
- Comprehensive docblock instructions at file top
- Clean code areas for implementation

**Reference Material** (`async-book-source/`)
- Complete source of the Rust Async Book
- Reference material I use for exercise content
- Examples and explanations for all concepts

## Exercise Design

### Structure Pattern

```rust
//! Exercise Title and Overview
//! 
//! Learning objectives and comprehensive instructions
//! Background from the async book
//! Implementation requirements
//! Examples and hints

// Clean implementation area

#[cfg(test)]
mod tests {
    // Comprehensive validation
}
```

### Content Guidelines

- All instructional content belongs in the docblock
- No inline comments in the implementation area
- Tests should validate understanding, not just syntax
- Each exercise builds on previous concepts

## User Experience Flow

1. User runs `cargo run -- next`
2. The tutorial generates the next available exercise
3. User implements solution following docblock instructions
4. User runs `cargo run -- check` for validation
5. All tests pass → next exercise unlocks automatically

## Technical Details

### Exercise Metadata

Each exercise contains:
- Unique identifier (e.g., "1.1", "2.2")
- Chapter and exercise number
- Title describing the concept
- Generator function producing the content

### Progress Tracking

`progress.json` maintains:
- Completed exercises map
- Current exercise identifier
- Automatically updated on test success

### Test-Based Completion

I've set it up to determine completion by:
1. Running exercise-specific tests
2. Checking all tests pass
3. No reliance on TODO markers or string matching
4. Automatic progression on success

## Development Workflow

### Adding New Exercises

1. Study relevant async book chapters
2. Create generator function in appropriate chapter module
3. Write comprehensive docblock instructions
4. Design tests that validate conceptual understanding
5. Test the complete flow from generation to completion

### Modifying Existing Exercises

1. Update generator function in `src/exercises/`
2. Ensure docblock remains comprehensive
3. Verify tests still validate core concepts
4. Test that progression flow remains intact

### Key Invariants

- Exercises must be completable with only the provided instructions
- Tests must be comprehensive enough to ensure understanding
- Each exercise should take 15-30 minutes for a competent Rust developer
- The progression should feel natural and educational (at least that's what I'm aiming for)

## Implementation Status

### Completed Features

- Single Cargo workspace architecture
- Progressive exercise generation system
- Test-based completion detection
- 10 exercises across 5 chapters
- CLI with all essential commands
- Persistent progress tracking

### Exercise Coverage

1. **Chapter 1** - Basic async/await and concurrent execution
2. **Chapter 2** - Future trait and custom executors
3. **Chapter 3** - Lifetime management in async contexts
4. **Chapter 4** - Multiple futures with join/select
5. **Chapter 5** - Async streams and iteration

## Working with the Codebase

### Important Files

- `src/main.rs` - CLI entry point and command handling
- `src/exercises/mod.rs` - Exercise registry and management
- `src/exercises/chapter*.rs` - Exercise generators by chapter
- `progress.json` - User progress (git-ignored)
- `exercises/` - Generated exercise files (git-ignored)

### Best Practices

1. Maintain consistency in exercise structure
2. Keep instructional content in docblocks only
3. Ensure tests are meaningful, not trivial
4. Reference async book content accurately
5. Test the full user experience flow

### Common Pitfalls

- Don't put comments in implementation areas
- Don't create exercises that require external knowledge
- Don't make tests that can pass without understanding
- Don't break the progressive unlocking system

## Future Considerations

The architecture I've built supports:
- Additional chapters and exercises
- Alternative progression paths
- Hint systems or guided mode
- Integration with language servers
- Performance benchmarking exercises

When extending this project, please maintain my core philosophy of progressive, test-driven learning with clean separation between instruction and implementation.