use anyhow::Result;
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    crossterm::{
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
};
use std::fs::File;

pub fn init() -> Result<Terminal<CrosstermBackend<File>>> {
    enable_raw_mode()?;

    let mut tty = File::create("/dev/tty")?;
    execute!(tty, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(tty);
    let terminal = Terminal::new(backend)?;

    Ok(terminal)
}

pub fn restore(terminal: &mut Terminal<CrosstermBackend<File>>) -> Result<()> {
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}
