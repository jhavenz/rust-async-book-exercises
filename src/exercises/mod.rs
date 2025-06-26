//! Exercise management module
//!
//! This module handles exercise generation, validation, and progression.
//! Each exercise is generated on-demand to keep the workspace clean.

use std::fs;
use std::path::Path;
use std::io::{self, Write};
use std::process::Command;
use colored::*;

pub mod chapter01;
pub mod chapter02;
pub mod chapter03;
pub mod chapter04;
pub mod chapter05;

#[derive(Debug, Clone)]
pub struct Exercise {
    pub id: String,
    pub title: String,
    pub chapter: u8,
    pub number: u8,
    pub generate: fn() -> String,
}

/// Get all available exercises
pub fn get_all_exercises() -> Vec<Exercise> {
    let mut exercises = vec![];

    exercises.extend(chapter01::get_exercises());
    exercises.extend(chapter02::get_exercises());
    exercises.extend(chapter03::get_exercises());
    exercises.extend(chapter04::get_exercises());
    exercises.extend(chapter05::get_exercises());

    exercises
}

/// List all exercises with their status
pub fn list_exercises(completed: &[String], current: &Option<String>) {
    // Clear screen for consistent presentation
    print!("\x1B[2J\x1B[1;1H");
    
    // Title
    println!("{}", "╔════════════════════════════════════════════════╗".bright_cyan());
    println!("{} {} {}", "║".bright_cyan(), "      🦀 Rust Async Book Tutorial 📚           ".bright_white().bold(), "║".bright_cyan());
    println!("{}", "╚════════════════════════════════════════════════╝".bright_cyan());
    println!();
    
    println!("{}", "📚 Exercise List".bright_yellow().bold());
    println!("   Complete all 10 exercises to master async Rust!");
    println!();

    let exercises = get_all_exercises();
    let mut current_chapter = 0;

    for ex in &exercises {
        if ex.chapter != current_chapter {
            current_chapter = ex.chapter;
            if current_chapter > 1 {
                println!();
            }
            println!("   {}", format!("Chapter {}", current_chapter).bright_yellow().bold());
        }

        let status = if completed.contains(&ex.id) {
            "✓".bright_green()
        } else if Some(&ex.id) == current.as_ref() {
            "→".bright_cyan()
        } else {
            " ".normal()
        };

        println!("   {} {} - {}", status, ex.id.bright_white(), ex.title);
    }
    
    println!();
    println!("{}", "─────────────────────────────────────────────────".bright_black());
    println!();
    
    // Show next action
    if let Some(current_id) = current {
        if let Some(exercise) = exercises.iter().find(|e| &e.id == current_id) {
            let filename = format!("exercises/ch{:02}_ex{:02}.rs", exercise.chapter, exercise.number);
            if Path::new(&filename).exists() {
                println!("   {} to test Exercise {}", "cargo run -- check".bright_green().bold(), current_id);
            } else {
                println!("   {} to generate Exercise {}", "cargo run -- next".bright_green().bold(), current_id);
            }
        }
    } else if completed.is_empty() {
        println!("   {} to begin!", "cargo run -- next".bright_green().bold());
    }
    
    println!();
}

/// Get total number of exercises
pub fn get_total_exercises() -> usize {
    get_all_exercises().len()
}

/// Check if an exercise ID is valid
pub fn is_valid_exercise(id: &str) -> bool {
    get_all_exercises().iter().any(|ex| ex.id == id)
}

/// Get the next exercise based on completed ones
pub fn get_next_exercise(completed: &[String]) -> Option<String> {
    let all_exercises = get_all_exercises();

    for ex in all_exercises {
        if !completed.contains(&ex.id) {
            return Some(ex.id);
        }
    }

    None
}

/// Generate exercise file
pub fn generate_exercise(id: &str) {
    let exercises = get_all_exercises();

    if let Some(exercise) = exercises.iter().find(|ex| ex.id == id) {
        // Create exercises directory if it doesn't exist
        fs::create_dir_all("exercises").ok();

        // Generate the exercise file
        let filename = format!("exercises/ch{:02}_ex{:02}.rs", exercise.chapter, exercise.number);
        let content = (exercise.generate)();

        fs::write(&filename, content).expect("Failed to write exercise file");
    }
}

/// Check if exercise passes all tests
pub fn check_exercise(id: &str) -> bool {
    let exercises = get_all_exercises();

    if let Some(exercise) = exercises.iter().find(|ex| ex.id == id) {
        let filename = format!("exercises/ch{:02}_ex{:02}.rs", exercise.chapter, exercise.number);

        if !Path::new(&filename).exists() {
            eprintln!("{}", "I can't find that exercise file. Try running the exercise first!".bright_red());
            return false;
        }

        // Create a temporary test project
        println!("{}", "🔧 Let me compile and test your solution...".bright_yellow());

        // Create temporary Cargo.toml for the exercise
        let temp_dir = "target/exercise_test";
        fs::create_dir_all(temp_dir).ok();

        let cargo_toml = format!(r#"[package]
name = "exercise_test"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "exercise"
path = "exercise.rs"

[dependencies]
tokio = {{ version = "1.0", features = ["full", "test-util"] }}
futures = "0.3"
"#);

        fs::write(format!("{}/Cargo.toml", temp_dir), cargo_toml).ok();
        fs::copy(&filename, format!("{}/exercise.rs", temp_dir)).ok();

        // First try to build - this will catch any todo!() macros or compilation errors
        let build_output = Command::new("cargo")
            .args(&["build", "--manifest-path", &format!("{}/Cargo.toml", temp_dir)])
            .output()
            .expect("Failed to run cargo build");

        if !build_output.status.success() {
            println!("{}", "❌ Your code doesn't compile yet:".bright_red());
            // Show both stdout and stderr for better error visibility
            if !build_output.stdout.is_empty() {
                println!("{}", String::from_utf8_lossy(&build_output.stdout));
            }
            if !build_output.stderr.is_empty() {
                eprintln!("{}", String::from_utf8_lossy(&build_output.stderr));
            }
            io::stdout().flush().ok();
            io::stderr().flush().ok();
            
            // Clean up
            fs::remove_dir_all(temp_dir).ok();
            return false;
        }

        // If it builds, run the tests
        let test_output = Command::new("cargo")
            .args(&["test", "--manifest-path", &format!("{}/Cargo.toml", temp_dir)])
            .output()
            .expect("Failed to run cargo test");

        // Clean up
        fs::remove_dir_all(temp_dir).ok();

        if test_output.status.success() {
            println!("{}", "✅ Excellent! All your tests are passing!".bright_green());
            true
        } else {
            println!("{}", "❌ Your tests aren't quite there yet:".bright_red());
            // Show both stdout and stderr for test output
            if !test_output.stdout.is_empty() {
                println!("{}", String::from_utf8_lossy(&test_output.stdout));
            }
            if !test_output.stderr.is_empty() {
                eprintln!("{}", String::from_utf8_lossy(&test_output.stderr));
            }
            io::stdout().flush().ok();
            io::stderr().flush().ok();
            false
        }
    } else {
        false
    }
}

/// Clean up all generated exercise files
pub fn cleanup_all() {
    if Path::new("exercises").exists() {
        fs::remove_dir_all("exercises").ok();
    }

    if Path::new("target/exercise_test").exists() {
        fs::remove_dir_all("target/exercise_test").ok();
    }
}

/// Macro to check if all tests in a module pass
#[macro_export]
macro_rules! exercise_complete {
    () => {{
        // This will be replaced by our test runner
        false
    }};
}
