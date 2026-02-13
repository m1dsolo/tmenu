mod app;
mod filterer;
mod tui;

use anyhow::Result;
use app::App;
use std::env;
use std::io::{self, BufRead};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();

    let stdin_options: Vec<String> = io::stdin()
        .lock()
        .lines()
        .filter_map(|line| line.ok())
        .filter(|line| !line.trim().is_empty())
        .collect();

    let mut options = args;
    options.extend(stdin_options);

    if options.is_empty() {
        eprintln!("No options provided on stdin.");
        // Exit cleanly if no options are provided
        return Ok(());
    }

    // Initialize the terminal
    let mut terminal = match tui::init() {
        Ok(term) => term,
        Err(err) => {
            eprintln!("Failed to initialize terminal: {err}");
            return Err(err);
        }
    };

    // Create and run the application
    let mut app = App::new(&options);
    let res = app.run(&mut terminal);

    tui::restore(&mut terminal)?;

    // Handle the result of the application
    match res {
        Ok(option) => {
            if let Some(option) = option {
                println!("{option}");
            }
            Ok(())
        }
        Err(err) => {
            eprintln!("Application error: {err}");
            Err(err.into())
        }
    }
}
