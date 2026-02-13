mod app;
mod filterer;
mod tui;

use anyhow::Result;
use app::App;
use clap::{Arg, Command};
use std::io::{self, BufRead};

fn main() -> Result<()> {
    let matches = Command::new("tmenu")
        .arg(
            Arg::new("fuzzy")
                .short('f')
                .long("fuzzy")
                .help("Use fuzzy matching")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(Arg::new("options").num_args(0..))
        .get_matches();

    let use_fuzzy = matches.get_flag("fuzzy");

    let args: Vec<String> = matches
        .get_many::<String>("options")
        .map(|vals| vals.cloned().collect())
        .unwrap_or_default();

    let stdin_options: Vec<String> = io::stdin()
        .lock()
        .lines()
        .filter_map(|line| line.ok())
        .filter(|line| !line.trim().is_empty())
        .collect();

    let mut options = args;
    options.extend(stdin_options);

    if options.is_empty() {
        eprintln!("No options provided.");
        return Ok(());
    }

    let mut terminal = match tui::init() {
        Ok(term) => term,
        Err(err) => {
            eprintln!("Failed to initialize terminal: {err}");
            return Err(err);
        }
    };

    let mut app = App::new(&options, use_fuzzy);
    let res = app.run(&mut terminal);

    tui::restore(&mut terminal)?;

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
