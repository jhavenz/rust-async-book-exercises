

use serde::{Serialize, Deserialize};
use std::fs;
use std::path::Path;
use crate::exercises;
use colored::*;

const PROGRESS_FILE: &str = ".async-book-progress.json";


#[derive(Debug, Serialize, Deserialize)]
pub struct Progress {
    pub completed_exercises: Vec<String>,
    pub current_exercise: Option<String>,
}

impl Default for Progress {
    fn default() -> Self {
        Progress {
            completed_exercises: vec![],
            current_exercise: Some("1.1".to_string()),
        }
    }
}

pub fn load_progress() -> Progress {
    if let Ok(content) = fs::read_to_string(PROGRESS_FILE) {
        serde_json::from_str(&content).unwrap_or_default()
    } else {
        Progress::default()
    }
}

pub fn save_progress(progress: &Progress) {
    let json = serde_json::to_string_pretty(progress).unwrap();
    fs::write(PROGRESS_FILE, json).unwrap();
}

pub fn show_progress(progress: &Progress) {
    // Clear screen for consistent presentation
    print!("\x1B[2J\x1B[1;1H");
    
    // Title
    println!("{}", "╔════════════════════════════════════════════════╗".bright_cyan());
    println!("{} {} {}", "║".bright_cyan(), "      🦀 Rust Async Book Tutorial 📚           ".bright_white().bold(), "║".bright_cyan());
    println!("{}", "╚════════════════════════════════════════════════╝".bright_cyan());
    println!();
    
    let total_exercises = exercises::get_total_exercises();
    let completed = progress.completed_exercises.len();
    let percentage = (completed as f32 / total_exercises as f32 * 100.0) as u32;
    
    // Progress Section
    println!("{}", "📊 Your Progress".bright_yellow().bold());
    print!("   ");
    let bar_width = 40;
    let filled = (bar_width * completed / total_exercises).max(0);
    print!("[");
    for i in 0..bar_width {
        if i < filled {
            print!("{}", "█".bright_green());
        } else {
            print!("{}", "░".bright_black());
        }
    }
    println!("] {}/{} ({}%)", completed, total_exercises, percentage);
    println!();
    
    // Current exercise
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
    } else if completed == total_exercises {
        println!("   {} All exercises completed!", "🎉".bright_green());
    }
    
    // Completed exercises
    if !progress.completed_exercises.is_empty() {
        println!();
        println!("{}", "✅ Completed Exercises".bright_yellow().bold());
        for ex_id in &progress.completed_exercises {
            if let Some(exercise) = exercises::get_all_exercises().iter().find(|e| e.id == *ex_id) {
                println!("   {} - {}", ex_id.bright_white(), exercise.title);
            }
        }
    }
    
    println!();
    println!("{}", "─────────────────────────────────────────────────".bright_black());
    println!();
    
    // Show next action
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
    }
    
    println!();
}

pub fn start_next_exercise(progress: &mut Progress) {
    if let Some(next) = exercises::get_next_exercise(&progress.completed_exercises) {
        progress.current_exercise = Some(next.clone());
        save_progress(progress);

        // Clear screen for consistent presentation
        print!("\x1B[2J\x1B[1;1H");
        
        // Title
        println!("{}", "╔════════════════════════════════════════════════╗".bright_cyan());
        println!("{} {} {}", "║".bright_cyan(), "      🦀 Rust Async Book Tutorial 📚           ".bright_white().bold(), "║".bright_cyan());
        println!("{}", "╚════════════════════════════════════════════════╝".bright_cyan());
        println!();

        // Generate the exercise
        exercises::generate_exercise(&next);
        
        if let Some(exercise) = exercises::get_all_exercises().iter().find(|e| e.id == next) {
            let filename = format!("exercises/ch{:02}_ex{:02}.rs", exercise.chapter, exercise.number);
            
            println!("{}", "📝 Exercise Generated".bright_yellow().bold());
            println!("   Exercise {}: {}", next.bright_white(), exercise.title);
            println!("   {} {}", "File".bright_blue(), filename.bright_white().underline());
            println!();
            
            println!("{}", "─────────────────────────────────────────────────".bright_black());
            println!();
            
            println!("   Open the file and replace the {} sections with your code.", "todo!()".bright_red());
            println!("   The instructions are in the file's documentation block.");
            println!();
            println!("   {} when you're ready to test your solution!", "cargo run -- check".bright_green().bold());
            println!();
        }
    } else {
        // Clear screen for consistent presentation
        print!("\x1B[2J\x1B[1;1H");
        
        // Title
        println!("{}", "╔════════════════════════════════════════════════╗".bright_cyan());
        println!("{} {} {}", "║".bright_cyan(), "      🦀 Rust Async Book Tutorial 📚           ".bright_white().bold(), "║".bright_cyan());
        println!("{}", "╚════════════════════════════════════════════════╝".bright_cyan());
        println!();
        
        println!("{}", "🎉 CONGRATULATIONS!".bright_green().bold());
        println!("   You've completed ALL the exercises!");
        println!();
        println!("   You've mastered the fundamentals of async Rust programming.");
        println!("   Time to build something amazing with your new skills! 🚀");
        println!();
    }
}

pub fn run_exercise(id: &str, progress: &mut Progress) {
    if exercises::is_valid_exercise(id) {
        progress.current_exercise = Some(id.to_string());
        save_progress(progress);

        // Clear screen for consistent presentation
        print!("\x1B[2J\x1B[1;1H");
        
        // Title
        println!("{}", "╔════════════════════════════════════════════════╗".bright_cyan());
        println!("{} {} {}", "║".bright_cyan(), "      🦀 Rust Async Book Tutorial 📚           ".bright_white().bold(), "║".bright_cyan());
        println!("{}", "╚════════════════════════════════════════════════╝".bright_cyan());
        println!();

        // Generate the exercise
        exercises::generate_exercise(id);
        
        if let Some(exercise) = exercises::get_all_exercises().iter().find(|e| e.id == id) {
            let filename = format!("exercises/ch{:02}_ex{:02}.rs", exercise.chapter, exercise.number);
            
            println!("{}", "📝 Exercise Generated".bright_yellow().bold());
            println!("   Exercise {}: {}", id.bright_white(), exercise.title);
            println!("   {} {}", "File".bright_blue(), filename.bright_white().underline());
            println!();
            
            println!("{}", "─────────────────────────────────────────────────".bright_black());
            println!();
            
            println!("   Open the file and replace the {} sections with your code.", "todo!()".bright_red());
            println!("   The instructions are in the file's documentation block.");
            println!();
            println!("   {} when you're ready to test your solution!", "cargo run -- check".bright_green().bold());
            println!();
        }
    } else {
        // Clear screen for consistent presentation
        print!("\x1B[2J\x1B[1;1H");
        
        // Title
        println!("{}", "╔════════════════════════════════════════════════╗".bright_cyan());
        println!("{} {} {}", "║".bright_cyan(), "      🦀 Rust Async Book Tutorial 📚           ".bright_white().bold(), "║".bright_cyan());
        println!("{}", "╚════════════════════════════════════════════════╝".bright_cyan());
        println!();
        
        println!("{}", "❌ Invalid Exercise".bright_red().bold());
        println!("   Exercise {} not found", id.bright_white());
        println!();
        println!("   {} to see all available exercises", "cargo run -- list".bright_cyan());
        println!();
    }
}

pub fn check_current_exercise(progress: &mut Progress) {
    if let Some(current) = progress.current_exercise.clone() {
        // Clear screen for consistent presentation
        print!("\x1B[2J\x1B[1;1H");
        
        // Title
        println!("{}", "╔════════════════════════════════════════════════╗".bright_cyan());
        println!("{} {} {}", "║".bright_cyan(), "      🦀 Rust Async Book Tutorial 📚           ".bright_white().bold(), "║".bright_cyan());
        println!("{}", "╚════════════════════════════════════════════════╝".bright_cyan());
        println!();
        
        println!("{}", "🧪 Testing Your Solution".bright_yellow().bold());
        println!("   Exercise {}", current.bright_white());
        println!();
        println!("{}", "─────────────────────────────────────────────────".bright_black());
        println!();

        if exercises::check_exercise(&current) {
            // Mark exercise as complete
            if !progress.completed_exercises.contains(&current) {
                progress.completed_exercises.push(current.clone());
            }
            
            // Get the next exercise
            if let Some(next) = exercises::get_next_exercise(&progress.completed_exercises) {
                progress.current_exercise = Some(next.clone());
                save_progress(progress);
                
                println!();
                println!("{}", "─────────────────────────────────────────────────".bright_black());
                println!();
                println!("{}", "🎉 Success!".bright_green().bold());
                println!("   You've completed Exercise {}!", current.bright_white());
                println!();
                println!("   {} Exercise {}", "Next up:".bright_cyan(), next.bright_white().bold());
                println!();
                println!("   {} to generate the next exercise!", "cargo run -- next".bright_green().bold());
                println!();
            } else {
                progress.current_exercise = None;
                save_progress(progress);
                
                println!();
                println!("{}", "─────────────────────────────────────────────────".bright_black());
                println!();
                println!("{}", "🎉 CONGRATULATIONS!".bright_green().bold());
                println!("   You've completed ALL the exercises!");
                println!();
                println!("   You've mastered the fundamentals of async Rust programming.");
                println!("   Time to build something amazing with your new skills! 🚀");
                println!();
            }
        } else {
            println!();
            println!("{}", "─────────────────────────────────────────────────".bright_black());
            println!();
            println!("{}", "💡 Keep Going!".bright_yellow().bold());
            println!("   Don't worry - async can be tricky at first!");
            println!("   Take another look at the error messages above.");
            println!("   Remember: the compiler is your friend! 🦀");
            println!();
        }
    } else {
        println!("No exercise loaded. Run `cargo run -- next` to get started.");
    }
}

pub fn reset_progress() {
    // Remove progress file
    if Path::new(PROGRESS_FILE).exists() {
        fs::remove_file(PROGRESS_FILE).ok();
    }

    // Clean up generated exercise files
    exercises::cleanup_all();

    // Clear screen for a fresh start
    print!("\x1B[2J\x1B[1;1H");
    
    // Title
    println!("{}", "╔════════════════════════════════════════════════╗".bright_cyan());
    println!("{} {} {}", "║".bright_cyan(), "      🦀 Rust Async Book Tutorial 📚           ".bright_white().bold(), "║".bright_cyan());
    println!("{}", "╚════════════════════════════════════════════════╝".bright_cyan());
    println!();
    
    // Reset status
    println!("{}", "✨ Progress Reset".bright_yellow().bold());
    println!("   All data cleared:");
    println!("   ✓ Progress file removed");
    println!("   ✓ Exercise files deleted");
    println!("   ✓ Test artifacts cleaned");
    println!();
    
    // Current status
    println!("{}", "📊 Your Progress".bright_yellow().bold());
    print!("   ");
    let bar_width = 40;
    print!("[");
    for _ in 0..bar_width {
        print!("{}", "░".bright_black());
    }
    println!("] 0/10 (0%)");
    println!("   {} Exercise 1.1: Basic Async/Await", "Ready for".bright_cyan());
    println!("   {} {}", "Will create".bright_blue(), "exercises/ch01_ex01.rs".bright_white().underline());
    println!();
    
    println!("{}", "─────────────────────────────────────────────────".bright_black());
    println!();
    
    // Next action
    println!("   {} to begin your async journey!", "cargo run -- next".bright_green().bold());
    println!();
}