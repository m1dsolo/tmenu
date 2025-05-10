use anyhow::Result;
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    crossterm::{
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
};
use std::io::{Stderr, stderr};

/// Initializes the terminal for Ratatui on stderr.
/// Enables raw mode and switches to the alternate screen buffer.
pub fn init() -> Result<Terminal<CrosstermBackend<Stderr>>> {
    enable_raw_mode()?;
    execute!(stderr(), EnterAlternateScreen)?;
    let terminal = Terminal::new(CrosstermBackend::new(stderr()))?;
    Ok(terminal)
}

/// Restores the terminal to its original state.
/// Disables raw mode and leaves the alternate screen buffer.
pub fn restore() -> Result<()> {
    disable_raw_mode()?;
    execute!(stderr(), LeaveAlternateScreen)?;
    Ok(())
}
