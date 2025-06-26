//! Chapter 4: Executing Multiple Futures at a Time
//! 
//! Now we're juggling multiple things at once - join!, select!, and spawning tasks.

use crate::exercises::Exercise;

pub fn get_exercises() -> Vec<Exercise> {
    vec![
        Exercise {
            id: "4.1".to_string(),
            title: "Join and Select".to_string(),
            chapter: 4,
            number: 1,
            generate: || "//! Exercise 4.1: Join and Select\n//! \n//! Working with multiple futures at once - join! waits for all,\n//! select! races to see who finishes first.\n//! \n//! ## What's happening here\n//! \n//! From Chapter 4 of the Rust Async Book:\n//! \n//! > Sometimes you need to run multiple futures concurrently.\n//! > Rust gives you different tools depending on what you need.\n//! \n//! It's like having multiple pots on the stove - sometimes you need\n//! them all to finish, sometimes just the first one.\n\n// Exercise content will be implemented\nfn main() {\n    println!(\"Exercise 4.1 - Coming soon!\");\n}\n".to_string(),
        },
        Exercise {
            id: "4.2".to_string(),
            title: "Spawning Tasks".to_string(),
            chapter: 4,
            number: 2,
            generate: || "//! Exercise 4.2: Spawning Tasks\n//! \n//! Sometimes you just want to fire off a task and let it do its thing\n//! in the background. That's where spawning comes in.\n//! \n//! ## Here's the scoop\n//! \n//! From Chapter 4 of the Rust Async Book:\n//! \n//! > Spawning lets you run futures in the background without\n//! > having to actively wait for them.\n//! \n//! Think of it like starting a washing machine - you don't stand\n//! there watching it, you go do other stuff.\n\n// Exercise content will be implemented\nfn main() {\n    println!(\"Exercise 4.2 - Coming soon!\");\n}\n".to_string(),
        },
    ]
}