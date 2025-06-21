# 🚀 Rust Async Programming Exercises

An interactive learning environment for mastering asynchronous programming in Rust, based on the [Async Book](https://rust-lang.github.io/async-book/).

## ✨ Features

- **🎯 Interactive CLI** - Simple commands to run, check, and test exercises
- **📊 Progress Tracking** - See what you've completed and what's next
- **🧪 Automated Testing** - Instant feedback on your solutions
- **💡 Guided Learning** - Clear TODO markers and helpful hints
- **🔄 Easy Reset** - Start over if you get stuck

## 🚀 Quick Start

```bash
# Get started immediately
./async-exercises list        # See all exercises
./async-exercises next        # Get your next exercise
./async-exercises run 1.1     # Start with the first exercise
```

## 📚 Exercise Overview

### Chapter 1: Async Fundamentals
- **1.1 Basic Async/Await** - Learn async/await syntax and concurrency
- **1.2 Concurrent Downloads** - Compare threads vs async performance

### Chapter 2: Under the Hood
- **2.1 Future Trait** - Implement custom Future types and understand polling
- **2.2 Custom Executor** - Build your own executor from scratch

## 🎮 How to Use

### 1. List Available Exercises
```bash
./async-exercises list
```

### 2. Work on an Exercise
```bash
./async-exercises run 1.1
```

### 3. Check Your Solution
```bash
./async-exercises check 1.1
```

### 4. Track Your Progress
```bash
./async-exercises progress
./async-exercises next
```

## 🎯 Exercise Workflow

1. **Run** an exercise to see the starter code
2. **Edit** the source file and implement the TODO sections
3. **Check** your solution for automatic validation
4. **Progress** automatically to the next exercise when complete

## 🛠️ Commands Reference

| Command | Description | Example |
|---------|-------------|---------|
| `list` | 📚 Show all exercises | `./async-exercises list` |
| `run <id>` | 🚀 Start working on an exercise | `./async-exercises run 1.1` |
| `check <id>` | 🔍 Validate your solution | `./async-exercises check 1.1` |
| `test <id>` | 🧪 Run automated tests | `./async-exercises test 1.1` |
| `progress` | 📊 Show completion status | `./async-exercises progress` |
| `next` | ⏭️ Get next exercise | `./async-exercises next` |
| `reset <id>` | 🔄 Reset to original state | `./async-exercises reset 1.1` |

## 🎯 What You'll Learn

### Core Concepts
- **Async vs Sync** - When and why to use async programming
- **Concurrency Model** - How async provides concurrency without threads
- **Future Trait** - The foundation of async Rust
- **Polling & Wakers** - Efficient scheduling mechanisms
- **Executors** - Runtime systems that drive futures

### Practical Skills
- Writing async functions with proper error handling
- Choosing between sequential and concurrent execution
- Implementing custom Future types
- Understanding performance characteristics
- Building simple executors

## 🧪 Testing System

Each exercise includes:
- **Compilation tests** - Ensure your code builds
- **Behavior tests** - Verify correct implementation
- **Performance tests** - Check concurrency and timing
- **Completeness tests** - Ensure all TODOs are implemented

## 📂 Project Structure

```
async-exercises/
├── exercises/
│   ├── chapter01/
│   │   ├── ex01_basic_async.rs
│   │   └── ex02_concurrent_download.rs
│   └── chapter02/
│       ├── ex01_future_trait.rs
│       └── ex02_custom_executor.rs
├── runner/                 # CLI tool source
├── async-exercises*        # Main executable
└── progress.json          # Your progress (auto-generated)
```

## 🛡️ No Dependency Hassles

Everything is pre-configured:
- ✅ Workspace setup with proper dependencies
- ✅ Tokio runtime configured  
- ✅ Testing framework integrated
- ✅ Build targets pre-defined

Just focus on learning async Rust!

## 🎓 Learning Path

1. **Start Simple** - Begin with basic async/await in Exercise 1.1
2. **See Benefits** - Experience performance gains in Exercise 1.2  
3. **Go Deeper** - Understand internals with Future trait in Exercise 2.1
4. **Master It** - Build your own executor in Exercise 2.2

## 💡 Tips for Success

- 🔍 Read TODO comments carefully - they contain implementation hints
- 🧪 Use `check` frequently to get instant feedback
- 📖 Refer to the [Async Book](https://rust-lang.github.io/async-book/) when stuck
- 🎯 Focus on understanding concepts, not just making tests pass
- 🔄 Don't hesitate to reset and try different approaches

## 🚨 Troubleshooting

**Exercise won't run?**
```bash
cargo check  # Check for compilation errors
```

**Lost track of progress?**
```bash
./async-exercises progress
./async-exercises next
```

**Want to start over?**
```bash
./async-exercises reset 1.1
```

## 🎉 Next Steps

After completing these exercises:
- 🌟 Build real async applications with [Tokio](https://tokio.rs/)
- 🌊 Explore async streams and iterators
- 🔗 Learn about async traits and advanced patterns
- 🚀 Contribute to async Rust projects

Ready to master async Rust? Let's begin! 

```bash
./async-exercises next
```