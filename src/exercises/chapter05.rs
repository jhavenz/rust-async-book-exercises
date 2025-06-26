//! Chapter 5: Streams
//! 
//! Async streams - like iterators, but async. Pretty straightforward once you get the hang of it.

use crate::exercises::Exercise;

pub fn get_exercises() -> Vec<Exercise> {
    vec![
        Exercise {
            id: "5.1".to_string(),
            title: "Basic Streams".to_string(),
            chapter: 5,
            number: 1,
            generate: || "//! Exercise 5.1: Basic Streams\n//! \n//! Time to play with async streams - they're like iterators that can\n//! wait for things.\n//! \n//! ## What's a stream anyway?\n//! \n//! From Chapter 5 of the Rust Async Book:\n//! \n//! > Streams are the async equivalent of iterators. Instead of\n//! > immediately having the next value, they might need to wait for it.\n//! \n//! Imagine a conveyor belt where items show up when they're ready,\n//! not all at once.\n\n// Exercise content will be implemented\nfn main() {\n    println!(\"Exercise 5.1 - Coming soon!\");\n}\n".to_string(),
        },
        Exercise {
            id: "5.2".to_string(),
            title: "Stream Processing".to_string(),
            chapter: 5,
            number: 2,
            generate: || "//! Exercise 5.2: Stream Processing\n//! \n//! Now that you've got the basics, let's do some cool stuff with streams -\n//! filtering, mapping, and all that jazz.\n//! \n//! ## Let's process some data\n//! \n//! From Chapter 5 of the Rust Async Book:\n//! \n//! > Processing streams efficiently is a key async skill. You can\n//! > transform, filter, and combine streams just like iterators.\n//! \n//! If you're comfortable with iterator adapters, this will feel familiar.\n\n// Exercise content will be implemented\nfn main() {\n    println!(\"Exercise 5.2 - Coming soon!\");\n}\n".to_string(),
        },
    ]
}