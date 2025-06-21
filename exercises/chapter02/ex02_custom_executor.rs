// Exercise 2.2: Building a Custom Executor
//
// This exercise walks you through building a simple executor from scratch,
// helping you understand how futures are actually executed.

use futures::task::{waker_ref, ArcWake, Context, Poll};
use futures::{Future, FutureExt};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::time::Duration;

// A task that can be executed by our executor
struct Task {
    future: Mutex<Option<Box<dyn Future<Output = ()> + Send + 'static>>>,
    task_sender: Arc<Mutex<VecDeque<Arc<Task>>>>,
}

impl Task {
    fn new(
        future: impl Future<Output = ()> + Send + 'static,
        task_sender: Arc<Mutex<VecDeque<Arc<Task>>>>,
    ) -> Arc<Self> {
        Arc::new(Task {
            future: Mutex::new(Some(Box::new(future))),
            task_sender,
        })
    }
}

// TODO: Implement ArcWake for Task
impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        // Your implementation here:
        // Add this task back to the task_sender queue so it gets polled again
        // 1. Clone arc_self
        // 2. Lock task_sender 
        // 3. Push the cloned task to the back of the queue
        
        todo!("Implement wake_by_ref to re-queue the task")
    }
}

// A simple executor that runs futures to completion
pub struct SimpleExecutor {
    ready_queue: Arc<Mutex<VecDeque<Arc<Task>>>>,
}

impl SimpleExecutor {
    pub fn new() -> Self {
        Self {
            ready_queue: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    // TODO: Implement the spawn method
    pub fn spawn(&mut self, future: impl Future<Output = ()> + Send + 'static) {
        // Your implementation here:
        // 1. Create a new Task using Task::new()
        // 2. Add it to the ready_queue
        
        todo!("Implement spawn method")
    }

    // TODO: Implement the run method
    pub fn run(&mut self) {
        // Your implementation here:
        // 1. Loop while there are tasks in ready_queue
        // 2. Pop a task from the front of the queue
        // 3. Lock the task's future
        // 4. If future exists, create a waker and context
        // 5. Poll the future
        // 6. If Poll::Ready, task is done
        // 7. If Poll::Pending, put future back (waker will re-queue task)
        
        todo!("Implement run method")
    }
}

// Example async function to test with
async fn example_task(id: usize, delay_ms: u64) {
    println!("📋 Task {} starting", id);
    
    for i in 0..3 {
        println!("📋 Task {} step {}", id, i);
        tokio::time::sleep(Duration::from_millis(delay_ms)).await;
    }
    
    println!("✅ Task {} completed", id);
}

// A counting task for testing
async fn counting_task(name: &str, max: usize) {
    for i in 1..=max {
        println!("🔢 {} counting: {}", name, i);
        tokio::time::sleep(Duration::from_millis(200)).await;
    }
    println!("🏁 {} finished counting to {}", name, max);
}

#[tokio::main]
async fn main() {
    println!("🚀 Welcome to Async Exercise 2.2!");
    println!("📖 Building a custom executor\n");
    
    println!("=== 🏃 Testing Simple Executor ===");
    let mut simple_executor = SimpleExecutor::new();
    
    // Spawn some tasks
    simple_executor.spawn(example_task(1, 100));
    simple_executor.spawn(example_task(2, 150));
    simple_executor.spawn(counting_task("Alpha", 3));
    
    // Run the executor
    simple_executor.run();

    println!("\n💡 Key concepts:");
    println!("   • Task scheduling and queuing");
    println!("   • Waker mechanism for efficient polling");
    println!("   • Future state management");
    println!("   • Executor event loops");
    
    println!("\n🤔 Think about:");
    println!("   • How does the wake mechanism prevent busy-waiting?");
    println!("   • What happens if a task never completes?");
    println!("   • How could you add priority scheduling?");
    println!("   • What are the tradeoffs vs thread-based approaches?");
    
    println!("\n✅ Run 'cargo run --bin run -- check 2.2' to verify your solution!");
}

// ==================== TESTS ====================
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::Instant;
    
    #[tokio::test]
    async fn test_simple_executor_runs_tasks() {
        let mut executor = SimpleExecutor::new();
        let counter = Arc::new(AtomicUsize::new(0));
        
        // Create a simple counting task
        let counter_clone = counter.clone();
        executor.spawn(async move {
            for _ in 0..5 {
                counter_clone.fetch_add(1, Ordering::SeqCst);
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
        });
        
        executor.run();
        
        assert_eq!(counter.load(Ordering::SeqCst), 5, 
            "Executor should run the task to completion");
    }
    
    #[tokio::test]
    async fn test_multiple_tasks_concurrent() {
        let mut executor = SimpleExecutor::new();
        let start_time = Instant::now();
        
        // Spawn multiple tasks that should run concurrently
        for i in 0..3 {
            executor.spawn(async move {
                println!("Task {} running", i);
                tokio::time::sleep(Duration::from_millis(100)).await;
            });
        }
        
        executor.run();
        
        let duration = start_time.elapsed();
        // Should complete in roughly 100ms (concurrent), not 300ms (sequential)
        assert!(duration < Duration::from_millis(200), 
            "Tasks should run concurrently, not sequentially");
    }
    
    #[test]
    fn test_no_todos_remaining() {
        let source = include_str!("ex02_custom_executor.rs");
        let todo_count = source.matches("todo!").count();
        
        assert_eq!(todo_count, 0, 
            "Remove all todo!() macros and implement the required functions");
    }
}