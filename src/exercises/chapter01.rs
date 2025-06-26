//! Chapter 1: Getting Started with Async
//! 
//! Starting with the basics - async/await syntax and why async code
//! can be so much faster than regular synchronous code.

use crate::exercises::Exercise;

pub fn get_exercises() -> Vec<Exercise> {
    vec![
        Exercise {
            id: "1.1".to_string(),
            title: "Basic Async/Await".to_string(),
            chapter: 1,
            number: 1,
            generate: generate_ex01,
        },
        Exercise {
            id: "1.2".to_string(),
            title: "Concurrent Downloads".to_string(),
            chapter: 1,
            number: 2,
            generate: generate_ex02,
        },
    ]
}

fn generate_ex01() -> String {
    r#"//! Exercise 1.1: Basic Async/Await
//! 
//! Time to get our feet wet with async/await syntax! This is where the fun begins.
//! 
//! ## Here's what we'll figure out
//! 
//! - How to convert regular functions to async ones
//! - The difference between doing things one-at-a-time vs all-at-once
//! - Why async code can be so much faster
//! 
//! ## Quick context
//! 
//! From Chapter 1 of the Rust Async Book:
//! 
//! > Async code allows us to write programs that can do multiple things at once.
//! > Traditional synchronous code executes one operation at a time, blocking until
//! > each completes. Async code can start multiple operations and switch between
//! > them as they wait for external resources.
//! 
//! The key insight is that many programs spend time waiting - for network requests,
//! disk I/O, or timers. Async programming lets us use that waiting time productively.
//! Pretty neat, right?
//! 
//! ## Let's dive in
//! 
//! Fill in the gaps below to convert synchronous functions to async
//! and implement concurrent execution. Don't worry if it feels weird at first -
//! async takes a bit to click.

use std::time::Duration;
use tokio::time::sleep;

/// Simulates learning a song (takes 1 second)
async fn learn_song() -> String {
    println!("🎵 Learning song...");
    sleep(Duration::from_millis(1000)).await;
    println!("📚 Song learned!");
    "Never Gonna Give You Up".to_string()
}

/// TODO: Convert this to an async function
/// 
/// Make this async so it can:
/// 1. Print "🎤 Singing: {song}"
/// 2. Sleep for 500ms using tokio::time::sleep
/// 3. Print "🎵 Finished singing!"
fn sing_song(song: String) {
    todo!("Convert this function to async")
}

/// TODO: Convert this to an async function
/// 
/// Same deal here - make it async and have it:
/// 1. Print "💃 Dancing!"
/// 2. Sleep for 1500ms
/// 3. Print "🕺 Finished dancing!"
fn dance() {
    todo!("Convert this function to async")
}

/// TODO: Implement this async function
/// 
/// Chain these together:
/// 1. Call learn_song() and await its result
/// 2. Call sing_song() with the learned song and await it
/// 
/// This shows how async functions can wait for each other - 
/// just like synchronous code, but with superpowers!
async fn learn_and_sing() {
    todo!("Implement sequential async execution")
}

/// TODO: Implement this async function
/// 
/// Here's where it gets cool - run learn_and_sing() and dance() 
/// at the same time!
/// 
/// Hint: Use tokio::join! macro to run multiple futures concurrently.
/// Example: tokio::join!(future1, future2);
async fn async_main() {
    todo!("Implement concurrent execution using join!")
}

#[tokio::main]
async fn main() {
    println!("🚀 Exercise 1.1: Basic Async/Await\n");
    
    // Once you implement the functions above, uncomment this code:
    /*
    println!("=== Sequential Execution ===");
    let start = std::time::Instant::now();
    
    let song = learn_song().await;
    sing_song(song).await;
    dance().await;
    
    println!("⏱️  Sequential took: {:?}\n", start.elapsed());
    
    println!("=== Concurrent Execution ===");
    let start = std::time::Instant::now();
    
    async_main().await;
    
    println!("⏱️  Concurrent took: {:?}", start.elapsed());
    
    println!("\n💡 Notice how concurrent execution is faster!");
    */
    
    println!("⚠️  Complete the TODO sections first!");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_functions_are_async() {
        // This test will only compile if the functions are properly async
        let _ = sing_song("test".to_string());
        let _ = dance();
        let _ = learn_and_sing();
        let _ = async_main();
    }
    
    #[tokio::test]
    async fn test_concurrent_is_faster() {
        use std::time::Instant;
        
        // Test sequential timing
        let start = Instant::now();
        learn_song().await;
        sing_song("test".to_string()).await;
        dance().await;
        let sequential_time = start.elapsed();
        
        // Test concurrent timing
        let start = Instant::now();
        async_main().await;
        let concurrent_time = start.elapsed();
        
        // Concurrent should be significantly faster
        assert!(concurrent_time < sequential_time);
        assert!(concurrent_time < Duration::from_millis(2000));
    }
    
}
"#.to_string()
}

fn generate_ex02() -> String {
    r#"//! Exercise 1.2: Concurrent Downloads
//! 
//! Now we're getting to the good stuff - seeing async really shine
//! with simulated network downloads.
//! 
//! ## What we're exploring
//! 
//! - How async makes I/O-bound operations way faster
//! - Getting comfortable with futures and concurrent execution
//! - A peek at how async runtimes work their magic
//! 
//! ## Here's the deal
//! 
//! From Chapter 1.2 of the Rust Async Book:
//! 
//! > In a typical threaded application, if you wanted to download two different
//! > webpages at the same time, you would spread the work across two threads.
//! > This is resource-intensive - threads have significant memory overhead.
//! > 
//! > With async, we can perform concurrent operations on a single thread,
//! > using far fewer resources while achieving similar performance.
//! 
//! ## Your turn
//! 
//! Build some download functions and see for yourself how much faster
//! concurrent downloads are compared to waiting for each one to finish.

use std::time::Duration;
use tokio::time::sleep;

/// Simulates downloading a file (takes between 500-1500ms)
async fn download_file(name: &str) -> String {
    println!("📥 Starting download: {}", name);
    
    // Simulate variable download time
    let duration = match name {
        "small.txt" => 500,
        "medium.jpg" => 1000,
        "large.zip" => 1500,
        _ => 1000,
    };
    
    sleep(Duration::from_millis(duration)).await;
    
    println!("✅ Completed download: {}", name);
    format!("Contents of {}", name)
}

/// TODO: Implement sequential downloads
/// 
/// Download these files one after another:
/// - "small.txt"
/// - "medium.jpg"  
/// - "large.zip"
/// 
/// Return a Vec<String> with all the downloaded contents.
/// Watch how long this takes!
async fn download_sequential() -> Vec<String> {
    todo!("Download files sequentially")
}

/// TODO: Implement concurrent downloads
/// 
/// Now download all three files at the same time using tokio::join!
/// 
/// Hint: You can destructure the join! result:
/// let (result1, result2, result3) = tokio::join!(future1, future2, future3);
/// 
/// This should be way faster than sequential - that's the power of async!
async fn download_concurrent() -> Vec<String> {
    todo!("Download files concurrently")
}

/// TODO: Implement a download with timeout
/// 
/// Try downloading "large.zip" but bail out if it takes more than 1 second.
/// 
/// Hint: Use tokio::time::timeout
/// Example: timeout(Duration::from_secs(1), some_future).await
/// 
/// Return Ok(content) if it finishes in time, Err("Download timed out") otherwise.
/// Sometimes you just can't wait forever, you know?
async fn download_with_timeout() -> Result<String, &'static str> {
    todo!("Implement download with timeout")
}

#[tokio::main]
async fn main() {
    println!("🚀 Exercise 1.2: Concurrent Downloads\n");
    
    // Uncomment after implementing the functions:
    /*
    println!("=== Sequential Downloads ===");
    let start = std::time::Instant::now();
    let files = download_sequential().await;
    println!("Downloaded {} files", files.len());
    println!("⏱️  Sequential took: {:?}\n", start.elapsed());
    
    println!("=== Concurrent Downloads ===");
    let start = std::time::Instant::now();
    let files = download_concurrent().await;
    println!("Downloaded {} files", files.len());
    println!("⏱️  Concurrent took: {:?}\n", start.elapsed());
    
    println!("=== Download with Timeout ===");
    match download_with_timeout().await {
        Ok(content) => println!("✅ Downloaded: {}", content),
        Err(e) => println!("❌ {}", e),
    }
    */
    
    println!("⚠️  Complete the TODO sections first!");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;
    
    #[tokio::test]
    async fn test_sequential_downloads() {
        let files = download_sequential().await;
        assert_eq!(files.len(), 3);
        assert!(files[0].contains("small.txt"));
        assert!(files[1].contains("medium.jpg"));
        assert!(files[2].contains("large.zip"));
    }
    
    #[tokio::test]
    async fn test_concurrent_is_faster() {
        let start = Instant::now();
        download_sequential().await;
        let sequential_time = start.elapsed();
        
        let start = Instant::now();
        download_concurrent().await;
        let concurrent_time = start.elapsed();
        
        // Concurrent should take about as long as the longest download (1.5s)
        // Sequential should take the sum of all downloads (3s)
        assert!(concurrent_time < Duration::from_millis(1700));
        assert!(sequential_time > Duration::from_millis(2500));
    }
    
    #[tokio::test]
    async fn test_timeout() {
        match download_with_timeout().await {
            Ok(_) => panic!("Should have timed out"),
            Err(msg) => assert_eq!(msg, "Download timed out"),
        }
    }
}
"#.to_string()
}