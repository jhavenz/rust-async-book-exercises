//! My Rust Async Book Tutorial
//!
//! This is my interactive Rust async programming tutorial based on the Rust Async Book.
//! I'm working through exercises to learn async/await concepts.
//!
//! ## Getting Started
//!
//! Run `cargo run` to start the tutorial.
//! Complete exercises by implementing the `todo!()` sections.
//! Tests will automatically validate your solutions.

use clap::{Parser, Subcommand};
use colored::*;

mod exercises;
mod progress;

#[derive(Parser)]
#[command(name = "async-book-exercises")]
#[command(about = "Rust Async Exercises", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// List all available exercises
    List,
    /// Show current progress
    Progress,
    /// Start or continue the next exercise
    Next,
    /// Run a specific exercise
    Run {
        /// Exercise ID (e.g., "1.1" or "2.3")
        id: String,
    },
    /// Check if current exercise passes all tests
    Check,
    /// Reset progress and start over
    Reset,
}

fn welcome_screen() {
    // Clear screen for a fresh start
    print!("\x1B[2J\x1B[1;1H");
    
    // Title
    println!("{}", "╔════════════════════════════════════════════════╗".bright_cyan());
    println!("{} {} {}", "║".bright_cyan(), "      🦀 Rust Async Book Tutorial 📚           ".bright_white().bold(), "║".bright_cyan());
    println!("{}", "╚════════════════════════════════════════════════╝".bright_cyan());
    println!();
    
    // Load progress data
    let progress = progress::load_progress();
    let total = exercises::get_total_exercises();
    let completed = progress.completed_exercises.len();
    let percentage = (completed as f32 / total as f32 * 100.0) as u32;
    
    // Progress Section
    println!("{}", "📊 Your Progress".bright_yellow().bold());
    print!("   ");
    let bar_width = 40;
    let filled = (bar_width * completed / total).max(0);
    print!("[");
    for i in 0..bar_width {
        if i < filled {
            print!("{}", "█".bright_green());
        } else {
            print!("{}", "░".bright_black());
        }
    }
    println!("] {}/{} ({}%)", completed, total, percentage);
    
    // Current status line
    if let Some(current) = &progress.current_exercise {
        if let Some(exercise) = exercises::get_all_exercises().iter().find(|e| e.id == *current) {
            let filename = format!("exercises/ch{:02}_ex{:02}.rs", exercise.chapter, exercise.number);
            println!("   {} Exercise {}: {}", "Working on".bright_cyan(), current.bright_white(), exercise.title);
            if std::path::Path::new(&filename).exists() {
                println!("   {} {}", "Edit".bright_blue(), filename.bright_white().underline());
            }
        }
    } else if completed == 0 {
        println!("   {}", "Ready to start your async journey!".bright_green());
    } else if completed == total {
        println!("   {} All exercises completed!", "🎉".bright_green());
    }
    
    println!();
    println!("{}", "─────────────────────────────────────────────────".bright_black());
    println!();
    
    // Quick command reference
    println!("   {}", "Commands:".bright_yellow());
    println!("   {} │ {} │ {} │ {} │ {}", 
        "next".bright_cyan().bold(),
        "check".bright_cyan().bold(),
        "list".bright_cyan().bold(),
        "progress".bright_cyan().bold(),
        "reset".bright_cyan().bold()
    );
    
    // Action prompt based on state
    println!();
    if let Some(current) = &progress.current_exercise {
        if let Some(exercise) = exercises::get_all_exercises().iter().find(|e| e.id == *current) {
            let filename = format!("exercises/ch{:02}_ex{:02}.rs", exercise.chapter, exercise.number);
            if std::path::Path::new(&filename).exists() {
                println!("   {} to test your solution", "cargo run -- check".bright_green().bold());
            } else {
                println!("   {} to generate the exercise", "cargo run -- next".bright_green().bold());
            }
        }
    } else if completed == 0 {
        println!("   {} to begin!", "cargo run -- next".bright_green().bold());
    } else {
        println!("   {} for more exercises", "cargo run -- next".bright_green().bold());
    }
    
    println!();
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::List) => {
            let progress = progress::load_progress();
            exercises::list_exercises(&progress.completed_exercises, &progress.current_exercise);
        }
        Some(Commands::Progress) => {
            let progress = progress::load_progress();
            progress::show_progress(&progress);
        }
        Some(Commands::Next) => {
            let mut progress = progress::load_progress();
            progress::start_next_exercise(&mut progress);
        }
        Some(Commands::Run { id }) => {
            let mut progress = progress::load_progress();
            progress::run_exercise(id, &mut progress);
        }
        Some(Commands::Check) => {
            let mut progress = progress::load_progress();
            progress::check_current_exercise(&mut progress);
        }
        Some(Commands::Reset) => {
            progress::reset_progress();
        }
        None => {
            // No command specified, show welcome screen
            welcome_screen();
        }
    }
}
