//! Chapter 3: async/.await
//! 
//! Getting into the trickier bits - lifetimes and move semantics in async land.

use crate::exercises::Exercise;

pub fn get_exercises() -> Vec<Exercise> {
    vec![
        Exercise {
            id: "3.1".to_string(),
            title: "Async Lifetimes".to_string(),
            chapter: 3,
            number: 1,
            generate: || "//! Exercise 3.1: Async Lifetimes\n//! \n//! Lifetimes in async can be a real head-scratcher. Let's figure them out together.\n//! \n//! ## Here's what's up\n//! \n//! From Chapter 3 of the Rust Async Book:\n//! \n//! > Lifetimes in async functions follow special rules because the generated\n//! > Future type needs to capture references safely.\n//! \n//! Don't worry, it clicks once you see the patterns.\n\n// Exercise content will be implemented\nfn main() {\n    println!(\"Exercise 3.1 - Coming soon!\");\n}\n".to_string(),
        },
        Exercise {
            id: "3.2".to_string(),
            title: "Async Move".to_string(),
            chapter: 3,
            number: 2,
            generate: || "//! Exercise 3.2: Async Move\n//! \n//! Time to tackle move semantics in async - when values need to move into\n//! async blocks and when they don't.\n//! \n//! ## The lowdown\n//! \n//! From Chapter 3 of the Rust Async Book:\n//! \n//! > Understanding when and how to move values into async blocks is crucial\n//! > for avoiding borrow checker fights.\n//! \n//! We've all been there with the borrow checker, right?\n\n// Exercise content will be implemented\nfn main() {\n    println!(\"Exercise 3.2 - Coming soon!\");\n}\n".to_string(),
        },
    ]
}