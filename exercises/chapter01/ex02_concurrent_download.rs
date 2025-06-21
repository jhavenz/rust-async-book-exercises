// Exercise 1.2: Concurrent Downloads
//
// This exercise demonstrates the performance benefits of async programming
// by implementing both threaded and async versions of concurrent downloads.

use futures::join;
use std::thread;
use std::time::{Duration, Instant};
use tokio::time::sleep;

// Simulates downloading data from a URL (blocking version)
fn download_sync(url: &str) -> String {
    println!("📡 Starting sync download from: {}", url);
    thread::sleep(Duration::from_millis(800)); // Simulate network delay
    let data = format!("Data from {}", url);
    println!("✅ Finished sync downloading from: {}", url);
    data
}

// TODO: Implement an async version of the download function
async fn download_async(url: &str) -> String {
    // Your implementation here:
    // 1. Print starting message
    // 2. Use tokio::time::sleep instead of thread::sleep  
    // 3. Create and return the data string
    // 4. Print finished message
    todo!("Implement async download function");
}

// Threaded version: downloads using OS threads
fn threaded_downloads() -> (String, String) {
    println!("=== 🧵 Threaded Downloads ===");
    let start = Instant::now();
    
    let handle1 = thread::spawn(|| download_sync("https://httpbin.org/delay/1"));
    let handle2 = thread::spawn(|| download_sync("https://jsonplaceholder.typicode.com/posts/1"));
    
    let result1 = handle1.join().unwrap();
    let result2 = handle2.join().unwrap();
    
    println!("⏱️  Threaded downloads took: {:?}\n", start.elapsed());
    (result1, result2)
}

// TODO: Implement async version of downloads using join!
async fn async_downloads() -> (String, String) {
    println!("=== ⚡ Async Downloads ===");
    let start = Instant::now();
    
    // Your implementation here:
    // 1. Create two futures by calling download_async  
    // 2. Use join! macro to run them concurrently
    // 3. Print the elapsed time
    // 4. Return the results
    
    todo!("Implement async downloads using join!")
}

// TODO: Implement sequential async downloads (for comparison)
async fn sequential_async_downloads() -> (String, String) {
    println!("=== 🐌 Sequential Async Downloads ===");
    let start = Instant::now();
    
    // Your implementation here:
    // 1. Call download_async for first URL and await it
    // 2. Call download_async for second URL and await it  
    // 3. Print elapsed time
    // 4. Return both results
    
    todo!("Implement sequential async downloads")
}

#[tokio::main]
async fn main() {
    println!("🚀 Welcome to Async Exercise 1.2!");
    println!("📖 Compare different concurrency approaches\n");
    
    // Run threaded version
    let _threaded_results = threaded_downloads();
    
    // Run async concurrent version
    let _async_results = async_downloads().await;
    
    // Run async sequential version  
    let _sequential_results = sequential_async_downloads().await;
    
    println!("💡 Key concepts:");
    println!("   • Threads vs async performance characteristics");
    println!("   • Memory overhead differences");
    println!("   • Concurrent vs sequential execution");
    println!("   • When to choose each approach");
    
    println!("\n🤔 Think about:");
    println!("   • Which approach was fastest and why?");
    println!("   • What happens with 1000 concurrent downloads?");
    println!("   • Memory usage of threads vs async tasks");
    
    println!("\n✅ Run 'cargo run --bin run -- check 1.2' to verify your solution!");
}

// ==================== TESTS ====================
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_download_async_implementation() {
        let start = Instant::now();
        let result = download_async("https://test.com").await;
        let duration = start.elapsed();
        
        assert_eq!(result, "Data from https://test.com");
        assert!(duration >= Duration::from_millis(700), 
            "download_async should simulate network delay");
        assert!(duration < Duration::from_millis(1000), 
            "download_async should not take too long");
    }
    
    #[tokio::test]
    async fn test_async_downloads_concurrent() {
        let start = Instant::now();
        let (result1, result2) = async_downloads().await;
        let duration = start.elapsed();
        
        assert!(result1.contains("httpbin.org"));
        assert!(result2.contains("jsonplaceholder"));
        
        // Should be concurrent (around 800ms), not sequential (1600ms)
        assert!(duration < Duration::from_millis(1200), 
            "Async downloads should run concurrently");
        assert!(duration >= Duration::from_millis(700), 
            "Should take at least as long as one download");
    }
    
    #[tokio::test]
    async fn test_sequential_async_downloads() {
        let start = Instant::now();
        let (result1, result2) = sequential_async_downloads().await;
        let duration = start.elapsed();
        
        assert!(result1.contains("httpbin.org"));
        assert!(result2.contains("jsonplaceholder"));
        
        // Should be sequential (around 1600ms)
        assert!(duration >= Duration::from_millis(1500), 
            "Sequential downloads should take longer");
    }
    
    #[test]
    fn test_no_todos_remaining() {
        let source = include_str!("ex02_concurrent_download.rs");
        let todo_count = source.matches("todo!").count();
        
        assert_eq!(todo_count, 0, 
            "Remove all todo!() macros and implement the required functions");
    }
}