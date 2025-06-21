use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
use colored::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Parser)]
#[command(name = "async-exercises")]
#[command(about = "Rust Async Programming Exercise Runner")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List all available exercises
    List,
    /// Run a specific exercise
    Run {
        /// Exercise identifier (e.g., 1.1, 2.2)
        exercise: String,
    },
    /// Check your solution for an exercise
    Check {
        /// Exercise identifier (e.g., 1.1, 2.2)
        exercise: String,
    },
    /// Run tests for an exercise
    Test {
        /// Exercise identifier (e.g., 1.1, 2.2)
        exercise: String,
    },
    /// Show your progress
    Progress,
    /// Reset an exercise to its original state
    Reset {
        /// Exercise identifier (e.g., 1.1, 2.2)
        exercise: String,
    },
    /// Show next exercise to work on
    Next,
}

#[derive(Serialize, Deserialize, Default)]
struct Progress {
    completed: HashMap<String, bool>,
    current: Option<String>,
}

#[derive(Clone)]
struct Exercise {
    id: String,
    name: String,
    chapter: u8,
    exercise_num: u8,
    description: String,
    path: String,
    test_command: Option<String>,
}

fn get_exercises() -> Vec<Exercise> {
    vec![
        Exercise {
            id: "1.1".to_string(),
            name: "Basic Async/Await".to_string(),
            chapter: 1,
            exercise_num: 1,
            description: "Learn the fundamentals of async/await syntax".to_string(),
            path: "exercises/chapter01/ex01_basic_async.rs".to_string(),
            test_command: Some("cargo test --package chapter01 --bin ex01_basic_async".to_string()),
        },
        Exercise {
            id: "1.2".to_string(),
            name: "Concurrent Downloads".to_string(),
            chapter: 1,
            exercise_num: 2,
            description: "Compare threaded vs async approaches".to_string(),
            path: "exercises/chapter01/ex02_concurrent_download.rs".to_string(),
            test_command: Some("cargo test --package chapter01 --bin ex02_concurrent_download".to_string()),
        },
        Exercise {
            id: "2.1".to_string(),
            name: "Future Trait".to_string(),
            chapter: 2,
            exercise_num: 1,
            description: "Implement custom Future types".to_string(),
            path: "exercises/chapter02/ex01_future_trait.rs".to_string(),
            test_command: Some("cargo test --package chapter02 --bin ex01_future_trait".to_string()),
        },
        Exercise {
            id: "2.2".to_string(),
            name: "Custom Executor".to_string(),
            chapter: 2,
            exercise_num: 2,
            description: "Build your own executor from scratch".to_string(),
            path: "exercises/chapter02/ex02_custom_executor.rs".to_string(),
            test_command: Some("cargo test --package chapter02 --bin ex02_custom_executor".to_string()),
        },
    ]
}

fn load_progress() -> Progress {
    if let Ok(content) = fs::read_to_string("progress.json") {
        serde_json::from_str(&content).unwrap_or_default()
    } else {
        Progress::default()
    }
}

fn save_progress(progress: &Progress) -> Result<()> {
    let content = serde_json::to_string_pretty(progress)?;
    fs::write("progress.json", content)?;
    Ok(())
}

fn find_exercise(id: &str) -> Option<Exercise> {
    get_exercises().into_iter().find(|ex| ex.id == id)
}

fn list_exercises() {
    let exercises = get_exercises();
    let progress = load_progress();
    
    println!("{}", "📚 Rust Async Programming Exercises".bold().blue());
    println!();
    
    let mut current_chapter = 0;
    for exercise in exercises {
        if exercise.chapter != current_chapter {
            current_chapter = exercise.chapter;
            println!("{}", format!("Chapter {}: Fundamentals", current_chapter).bold().yellow());
        }
        
        let status = if *progress.completed.get(&exercise.id).unwrap_or(&false) {
            "✅".green()
        } else if progress.current.as_ref() == Some(&exercise.id) {
            "🔄".yellow()
        } else {
            "⏳".red()
        };
        
        println!("  {} {} - {}", status, exercise.id.bold(), exercise.name);
        println!("      {}", exercise.description.dimmed());
    }
    
    println!();
    println!("{}", "Commands:".bold());
    println!("  {} run <id>     - Work on an exercise", "📝".green());
    println!("  {} check <id>   - Check your solution", "🔍".blue());
    println!("  {} test <id>    - Run tests", "🧪".yellow());
    println!("  {} progress     - Show progress", "📊".cyan());
    println!("  {} next         - Get next exercise", "⏭️".magenta());
}

fn run_exercise(id: &str) -> Result<()> {
    let exercise = find_exercise(id).ok_or_else(|| anyhow!("Exercise {} not found", id))?;
    
    println!("{}", format!("🚀 Running Exercise {}: {}", exercise.id, exercise.name).bold().green());
    println!("{}", exercise.description.dimmed());
    println!();
    
    // Update progress
    let mut progress = load_progress();
    progress.current = Some(exercise.id.clone());
    save_progress(&progress)?;
    
    // Run the exercise
    let binary_name = match exercise.id.as_str() {
        "1.1" => "ex01_basic_async",
        "1.2" => "ex02_concurrent_download", 
        "2.1" => "ex01_future_trait",
        "2.2" => "ex02_custom_executor",
        _ => return Err(anyhow!("Unknown exercise: {}", exercise.id)),
    };
    
    let status = Command::new("cargo")
        .args(&["run", "--package", &format!("chapter{:02}", exercise.chapter), "--bin", binary_name])
        .status()?;
    
    if !status.success() {
        println!("{}", "❌ Exercise failed to run. Check for compilation errors.".red());
        return Ok(());
    }
    
    println!();
    println!("{}", "💡 Hints:".yellow().bold());
    println!("  • Look for TODO comments in the code");
    println!("  • Read the inline documentation carefully");
    println!("  • Use `check {}` to validate your solution", exercise.id);
    println!("  • Use `test {}` to run automated tests", exercise.id);
    
    Ok(())
}

fn check_exercise(id: &str) -> Result<()> {
    let exercise = find_exercise(id).ok_or_else(|| anyhow!("Exercise {} not found", id))?;
    
    println!("{}", format!("🔍 Checking Exercise {}: {}", exercise.id, exercise.name).bold().blue());
    
    // Check if the file exists and has been modified
    if !Path::new(&exercise.path).exists() {
        println!("{}", "❌ Exercise file not found!".red());
        return Ok(());
    }
    
    // Read the file and check for TODO markers
    let content = fs::read_to_string(&exercise.path)?;
    let todo_count = content.matches("todo!").count();
    
    if todo_count > 0 {
        println!("{}", format!("⚠️  Found {} unimplemented TODO sections", todo_count).yellow());
        println!("   Keep working on your implementation!");
        return Ok(());
    }
    
    // Try to compile the exercise
    let binary_name = match exercise.id.as_str() {
        "1.1" => "ex01_basic_async",
        "1.2" => "ex02_concurrent_download", 
        "2.1" => "ex01_future_trait",
        "2.2" => "ex02_custom_executor",
        _ => return Err(anyhow!("Unknown exercise: {}", exercise.id)),
    };
    
    let compile_result = Command::new("cargo")
        .args(&["check", "--package", &format!("chapter{:02}", exercise.chapter), "--bin", binary_name])
        .output()?;
    
    if !compile_result.status.success() {
        println!("{}", "❌ Compilation failed:".red());
        println!("{}", String::from_utf8_lossy(&compile_result.stderr));
        return Ok(());
    }
    
    println!("{}", "✅ Code compiles successfully!".green());
    
    // Run tests if available  
    println!("{}", "🧪 Running tests...".yellow());
    let test_result = Command::new("cargo")
        .args(&["test", "--package", &format!("chapter{:02}", exercise.chapter), "--bin", binary_name])
        .output()?;
        
        if test_result.status.success() {
            println!("{}", "✅ All tests passed!".green().bold());
            
            // Mark as completed
            let mut progress = load_progress();
            progress.completed.insert(exercise.id.clone(), true);
            progress.current = find_next_exercise(&exercise.id).map(|ex| ex.id);
            save_progress(&progress)?;
            
            println!("{}", "🎉 Exercise completed!".green().bold());
            
            if let Some(next) = find_next_exercise(&exercise.id) {
                println!("📚 Next up: Exercise {} - {}", next.id.bold(), next.name);
                println!("   Run: {} run {}", "cargo".cyan(), next.id.cyan());
            } else {
                println!("🏆 Congratulations! You've completed all exercises!");
            }
        } else {
            println!("{}", "❌ Some tests failed:".red());
            println!("{}", String::from_utf8_lossy(&test_result.stdout));
            println!("{}", String::from_utf8_lossy(&test_result.stderr));
        }
    
    Ok(())
}

fn show_progress() {
    let exercises = get_exercises();
    let progress = load_progress();
    
    let completed_count = progress.completed.values().filter(|&&v| v).count();
    let total_count = exercises.len();
    
    println!("{}", "📊 Your Progress".bold().cyan());
    println!();
    println!("Completed: {}/{} ({}%)", 
        completed_count.to_string().green().bold(),
        total_count,
        (completed_count * 100 / total_count).to_string().green().bold()
    );
    
    if let Some(current) = &progress.current {
        if let Some(exercise) = find_exercise(current) {
            println!("Currently working on: {} - {}", 
                exercise.id.yellow().bold(), 
                exercise.name.yellow()
            );
        }
    }
    
    println!();
    
    // Show detailed progress
    let mut current_chapter = 0;
    for exercise in exercises {
        if exercise.chapter != current_chapter {
            current_chapter = exercise.chapter;
            println!("Chapter {}:", current_chapter);
        }
        
        let status = if *progress.completed.get(&exercise.id).unwrap_or(&false) {
            "✅ Completed".green()
        } else if progress.current.as_ref() == Some(&exercise.id) {
            "🔄 In Progress".yellow()
        } else {
            "⏳ Not Started".red()
        };
        
        println!("  {} {} - {}", exercise.id.bold(), exercise.name, status);
    }
}

fn find_next_exercise(current_id: &str) -> Option<Exercise> {
    let exercises = get_exercises();
    let current_idx = exercises.iter().position(|ex| ex.id == current_id)?;
    exercises.get(current_idx + 1).cloned()
}

fn show_next() {
    let progress = load_progress();
    
    if let Some(current) = &progress.current {
        if let Some(exercise) = find_exercise(current) {
            println!("{}", format!("🔄 Currently working on: {} - {}", exercise.id, exercise.name).yellow());
            println!("   Run: {} run {}", "cargo".cyan(), exercise.id.cyan());
            return;
        }
    }
    
    // Find first incomplete exercise
    let exercises = get_exercises();
    for exercise in exercises {
        if !progress.completed.get(&exercise.id).unwrap_or(&false) {
            println!("{}", format!("📚 Next exercise: {} - {}", exercise.id, exercise.name).green().bold());
            println!("   {}", exercise.description.dimmed());
            println!("   Run: {} run {}", "cargo".cyan(), exercise.id.cyan());
            return;
        }
    }
    
    println!("{}", "🏆 All exercises completed! Great job!".green().bold());
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::List => list_exercises(),
        Commands::Run { exercise } => run_exercise(&exercise)?,
        Commands::Check { exercise } => check_exercise(&exercise)?,
        Commands::Test { exercise } => {
            // Test command implementation
            println!("Running tests for exercise {}", exercise);
        }
        Commands::Progress => show_progress(),
        Commands::Reset { exercise: _ } => {
            println!("Reset functionality not yet implemented");
        }
        Commands::Next => show_next(),
    }
    
    Ok(())
}