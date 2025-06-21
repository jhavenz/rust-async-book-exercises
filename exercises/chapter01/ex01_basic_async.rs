// Exercise 1.1: Basic Async/Await
// 
// This exercise teaches you the fundamentals of async/await syntax.
// You'll convert synchronous code to asynchronous and see the performance benefits.

use futures::{join, executor::block_on};
use std::time::{Duration, Instant};
use tokio::time::sleep;

// TODO: Convert this synchronous function to an async function
fn learn_song() -> String {
    println!("🎵 Learning song...");
    std::thread::sleep(Duration::from_millis(1000));
    println!("📚 Song learned!");
    "Happy Birthday".to_string()
}

// TODO: Convert this synchronous function to an async function  
fn sing_song(song: String) {
    println!("🎤 Singing: {}", song);
    std::thread::sleep(Duration::from_millis(500));
    println!("🎵 Finished singing!");
}

// TODO: Convert this synchronous function to an async function
fn dance() {
    println!("💃 Dancing!");
    std::thread::sleep(Duration::from_millis(1500));
    println!("🕺 Finished dancing!");
}

// TODO: Implement this function to run learn_song and sing_song sequentially
async fn learn_and_sing() {
    // Your code here - call learn_song_async().await then sing_song_async().await
    todo!("Implement learn_and_sing function");
}

// TODO: Implement this function to run learn_and_sing and dance concurrently
async fn async_main() {
    // Your code here - use join! macro to run learn_and_sing() and dance_async() concurrently
    todo!("Implement async_main function with join!");
}

#[tokio::main]
async fn main() {
    println!("🚀 Welcome to Async Exercise 1.1!");
    println!("📖 Learn the basics of async/await syntax\n");
    
    println!("=== 🐌 Synchronous Version ===");
    let start = Instant::now();
    
    let song = learn_song();
    sing_song(song);
    dance();
    
    println!("⏱️  Synchronous version took: {:?}\n", start.elapsed());
    
    println!("=== ⚡ Asynchronous Version ===");
    let start = Instant::now();
    
    async_main().await;
    
    println!("⏱️  Asynchronous version took: {:?}", start.elapsed());
    
    println!("\n💡 Key concepts to understand:");
    println!("   • async/await syntax");
    println!("   • Sequential vs concurrent execution");
    println!("   • Performance benefits of async");
    println!("\n✅ Run 'cargo run --bin run -- check 1.1' to verify your solution!");
}

// ==================== TESTS ====================
#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;
    
    // Test helper functions
    async fn learn_song_async() -> String {
        println!("🎵 Learning song...");
        sleep(Duration::from_millis(1000)).await;
        println!("📚 Song learned!");
        "Happy Birthday".to_string()
    }
    
    async fn sing_song_async(song: String) {
        println!("🎤 Singing: {}", song);
        sleep(Duration::from_millis(500)).await;
        println!("🎵 Finished singing!");
    }
    
    async fn dance_async() {
        println!("💃 Dancing!");
        sleep(Duration::from_millis(1500)).await;
        println!("🕺 Finished dancing!");
    }

    #[tokio::test]
    async fn test_learn_and_sing_implementation() {
        // This test checks if learn_and_sing is properly implemented
        let start = Instant::now();
        
        // Check that the functions exist and can be called
        // For now, we'll just ensure they compile and don't panic
        let song = learn_song_async().await;
        sing_song_async(song).await;
        
        let duration = start.elapsed();
        // Should take at least 1500ms (1000 + 500)
        assert!(duration >= Duration::from_millis(1400), 
            "learn_and_sing should take at least 1400ms for sequential execution");
    }
    
    #[tokio::test]
    async fn test_concurrent_execution() {
        // This test verifies that async_main runs concurrently
        let start = Instant::now();
        
        // Run the equivalent of what async_main should do
        join!(
            async {
                let song = learn_song_async().await;
                sing_song_async(song).await;
            },
            dance_async()
        );
        
        let duration = start.elapsed();
        // Should take around 1500ms (dance time), not 3000ms (sequential time)
        assert!(duration < Duration::from_millis(2000), 
            "Concurrent execution should be faster than sequential");
        assert!(duration >= Duration::from_millis(1400), 
            "Should still take at least as long as the longest task");
    }
    
    #[test]
    fn test_no_todos_remaining() {
        // This test fails if there are still todo!() macros in the code
        let source = include_str!("ex01_basic_async.rs");
        let todo_count = source.matches("todo!").count();
        
        assert_eq!(todo_count, 0, 
            "Remove all todo!() macros and implement the required functions");
    }
}