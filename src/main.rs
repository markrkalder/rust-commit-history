mod scanner;

use std::{
    fs::{File, OpenOptions},
    io::Write,
};

use clap::{Parser, Subcommand};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Add a directory to the git history
    Add {
        /// Path of the directory to add
        dir_path: String,
    },
    /// Remove a directory from the git history
    Rm {
        /// Path of the directory to remove
        dir_path: String,
    },
    /// Show the commit history
    Show,
}

fn main() {
    let cli = Cli::parse();

    // check if the home directory is accessible
    let home_dir = match home::home_dir() {
        Some(dir) => dir,
        None => {
            eprintln!("Unable to get home dir.");
            return;
        }
    };

    let stats_path = home_dir.join(".gitlocalstats");

    if !stats_path.exists() {
        println!("creating dotfile");
        let _ = File::create(&stats_path);
    }

    // try to open the dotfile
    let mut file = match OpenOptions::new().append(true).open(&stats_path) {
        Ok(f) => {
            println!("File opened successfully: {:?}", stats_path);
            f
        }
        Err(e) => {
            eprintln!("Failed to open the file: {}", e);
            return;
        }
    };

    let excluded = ["node_modules", "vendor", ".git"];

    match &cli.command {
        Commands::Add { dir_path } => {
            let dirs = scanner::scan_dir(dir_path, &excluded);
            for dir in dirs {
                println!("Scanned dir {}", dir.display());
                if let Err(e) = file.write_all(format!("{}\n", dir.display()).as_bytes()) {
                    eprintln!("Error writing to file: {}", e);
                }
            }
        }
        Commands::Rm { dir_path } => {
            let dirs = scanner::scan_dir(dir_path, &excluded);
            for dir in dirs {
                println!("Removing directories: {}", dir.display());
            }
        }
        Commands::Show => {
            println!("Showing git commit history");
        }
    }
}
