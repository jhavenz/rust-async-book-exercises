//! Chapter 2: Under the Hood - Future and Task
//! 
//! Ready to peek under the hood? We're going to see how async/await
//! actually works, including the Future trait and executor basics.

use crate::exercises::Exercise;

pub fn get_exercises() -> Vec<Exercise> {
    vec![
        Exercise {
            id: "2.1".to_string(),
            title: "The Future Trait".to_string(),
            chapter: 2,
            number: 1,
            generate: generate_ex01,
        },
        Exercise {
            id: "2.2".to_string(),
            title: "Custom Executor".to_string(),
            chapter: 2,
            number: 2,
            generate: generate_ex02,
        },
    ]
}

fn generate_ex01() -> String {
    r#"//! Exercise 2.1: The Future Trait
//! 
//! Time to see what makes async tick by building our own futures!
//! 
//! ## Quick context
//! 
//! From Chapter 2.1 of the Rust Async Book:
//! 
//! > A Future is an asynchronous computation that can produce a value.
//! > The Future trait is at the core of asynchronous programming in Rust.
//! 
//! Think of it like a promise that something will happen - eventually.

// Exercise content will be implemented
fn main() {
    println!("Exercise 2.1 - Coming soon!");
}
"#.to_string()
}

fn generate_ex02() -> String {
    r#"//! Exercise 2.2: Custom Executor
//! 
//! Ever wonder how futures actually get run? Let's build a simple
//! executor and find out!

fn main() {
    println!("Exercise 2.2 - Coming soon!");
}
"#.to_string()
}