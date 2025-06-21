# Claude Context: Rust Async Programming Exercise System

## 🎯 Project Overview

This is an **interactive learning system for Rust async programming** designed to teach developers asynchronous programming concepts through hands-on exercises. The system provides a CLI-driven experience with automated testing and progress tracking.

## 🏗️ System Architecture

### **Core Components:**

1. **CLI Runner** (`runner/src/main.rs`)
   - Interactive command-line interface for managing exercises
   - Progress tracking with JSON persistence
   - Automated testing and validation
   - User-friendly feedback and guidance

2. **Exercise Workspace** (`exercises/`)
   - Structured learning modules organized by chapter
   - Each exercise is a standalone Rust binary with tests
   - Clear TODO markers guide implementation
   - Progressive difficulty building understanding

3. **Reference Materials** (`async-book-source/`)
   - Complete source code of the official Rust Async Book
   - Provides authoritative context for exercise creation
   - Contains examples and explanations for all async concepts
   - Serves as reference when extending or debugging exercises

## 📚 Exercise Structure

### **Current Exercises:**

**Chapter 1: Async Fundamentals**
- `ex01_basic_async.rs` - Learn async/await syntax and concurrency patterns
- `ex02_concurrent_download.rs` - Compare threaded vs async performance

**Chapter 2: Under the Hood** 
- `ex01_future_trait.rs` - Implement custom Future types and understand polling
- `ex02_custom_executor.rs` - Build a custom executor from scratch

### **Exercise Pattern:**
Each exercise follows this structure:
```rust
// Educational context and objectives
// TODO: Implementation sections with clear guidance
// main() function demonstrating concepts
// #[cfg(test)] module with automated validation
```

## 🎮 User Workflow

**Primary Commands:**
```bash
./async-exercises list      # Show all exercises
./async-exercises next      # Get next exercise to work on  
./async-exercises run 1.1   # Start exercise 1.1
./async-exercises check 1.1 # Validate solution with tests
./async-exercises progress  # Show completion status
```

**Learning Flow:**
1. User runs an exercise to see starter code
2. User edits the exercise file, implementing TODO sections
3. User checks solution for immediate feedback
4. System automatically progresses to next exercise when complete

## 🧪 Testing System

**Multi-layered Validation:**
- **Compilation Tests** - Ensure code builds without errors
- **Behavior Tests** - Verify correct implementation semantics
- **Performance Tests** - Confirm async benefits (timing, concurrency)
- **Completeness Tests** - Check all TODO markers are implemented

**Test Integration:**
- Each exercise has embedded `#[cfg(test)]` modules
- Tests run via `cargo test --package chapterXX --bin exerciseYY`
- Clear pass/fail feedback with descriptive error messages

## 📁 Directory Structure

```
async-exercises/
├── async-exercises*           # Main CLI executable script
├── Cargo.toml                # Workspace configuration
├── README.md                 # User-facing documentation
├── GETTING_STARTED.md        # Quick start guide
├── CLAUDE.md                 # This file - Claude context
├── progress.json             # User progress tracking (auto-generated)
│
├── runner/                   # CLI tool source code
│   ├── Cargo.toml
│   └── src/main.rs          # Main CLI implementation
│
├── exercises/                # Exercise workspace
│   ├── chapter01/           # Async fundamentals
│   │   ├── Cargo.toml      
│   │   ├── lib.rs          
│   │   ├── ex01_basic_async.rs
│   │   └── ex02_concurrent_download.rs
│   └── chapter02/           # Advanced concepts
│       ├── Cargo.toml      
│       ├── lib.rs          
│       ├── ex01_future_trait.rs
│       └── ex02_custom_executor.rs
│
└── async-book-source/        # Official Rust Async Book source
    ├── src/                 # Book chapters in Markdown
    │   ├── 01_getting_started/
    │   ├── 02_execution/
    │   ├── 03_async_await/
    │   └── ...
    └── examples/            # Official book examples
        ├── 01_02_why_async/
        ├── 02_02_future_trait/
        └── ...
```

## 🔍 Why async-book-source Exists

**Primary Purposes:**

1. **Authoritative Reference** - Contains the complete, official Rust Async Book source code that serves as the canonical reference for async concepts

2. **Exercise Foundation** - All exercises are based on concepts and examples from this book, ensuring alignment with official Rust async guidance

3. **Context for Claude** - When working on this project, Claude can reference:
   - Book chapters for conceptual understanding
   - Official examples for implementation patterns  
   - Progression and difficulty ordering from the book structure

4. **Future Development** - Enables creation of additional exercises based on book content:
   - Streams and async iteration (Chapter 5)
   - Multiple futures and combinators (Chapter 6) 
   - Advanced patterns and workarounds (Chapter 7)
   - Real-world examples (Chapter 9)

**Key Reference Files:**
- `async-book-source/src/SUMMARY.md` - Complete book structure
- `async-book-source/src/01_getting_started/` - Foundation concepts
- `async-book-source/src/02_execution/` - Future trait and executors
- `async-book-source/examples/` - Working code examples

## 🛠️ Development Guidelines

**When Modifying Exercises:**
1. Ensure exercises build and tests pass: `cargo test`
2. Maintain clear TODO markers with implementation hints
3. Keep exercises focused on specific learning objectives
4. Reference async-book-source for concept accuracy

**When Adding New Exercises:**
1. Study relevant chapters in `async-book-source/src/`
2. Follow existing exercise patterns and naming conventions
3. Add appropriate test coverage and validation
4. Update CLI runner with new exercise metadata

**When Debugging Issues:**
1. Check `async-book-source/examples/` for reference implementations
2. Review book chapters for conceptual guidance
3. Ensure workspace dependencies are correctly configured
4. Verify CLI runner handles new exercises properly

## 🎯 Learning Objectives

**Chapter 1 Goals:**
- Understand async/await syntax and semantics
- Compare sync vs async performance characteristics
- Learn when to use async vs threads
- Master basic concurrency patterns

**Chapter 2 Goals:**  
- Understand the Future trait and polling model
- Learn about Wakers and efficient scheduling
- Implement custom Future types
- Build simple executors from scratch

## 🚀 Usage for Claude

When working with this project:

1. **For Exercise Questions** - Reference `async-book-source/src/` chapters for authoritative explanations

2. **For Implementation Help** - Check `async-book-source/examples/` for working code patterns

3. **For New Exercise Ideas** - Review book structure in `SUMMARY.md` for additional topics

4. **For Concept Verification** - Use book source as ground truth for async Rust best practices

5. **For Debugging** - Exercise test failures can be understood by referencing the corresponding book concepts

This system provides a complete, self-contained learning environment for Rust async programming with built-in guidance, testing, and reference materials.