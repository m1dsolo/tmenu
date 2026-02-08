use anyhow::Result;
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, read},
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};
use std::io::Stderr;

pub struct App<'a> {
    running: bool,
    input: String,
    all_options: Vec<&'a str>,
    filtered_options: Vec<&'a str>,
    list_state: ListState,
    confirmed: bool,
}

impl<'a> App<'a> {
    pub fn new(options: &'a [String]) -> Self {
        assert!(!options.is_empty(), "Options cannot be empty");

        let options_refs = options
            .iter()
            .map(|option| option.as_str())
            .collect::<Vec<_>>();

        let mut list_state = ListState::default();
        if !options_refs.is_empty() {
            list_state.select(Some(0)); // Select the first item initially
        }

        Self {
            running: true,
            input: String::new(),
            all_options: options_refs.clone(),
            filtered_options: options_refs,
            list_state,
            confirmed: false,
        }
    }

    pub fn run(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<Stderr>>,
    ) -> Result<Option<&'a str>> {
        while self.running {
            terminal.draw(|frame| self.draw(frame))?;
            if event::poll(std::time::Duration::from_millis(50))? {
                if let Event::Key(key_event) = read()? {
                    self.handle_event(key_event);
                }
            }
        }

        Ok(self.get_result())
    }

    fn filter_options(&mut self) {
        let input_lower = self.input.to_lowercase();
        self.filtered_options = self
            .all_options
            .iter()
            .filter(|option| option.to_lowercase().contains(&input_lower))
            .copied()
            .collect();

        // After filtering, reset selection to the first item if there are results
        self.list_state
            .select(self.filtered_options.first().map(|_| 0));
    }

    fn select_next(&mut self) {
        self.list_state.select_next();
    }

    fn select_previous(&mut self) {
        self.list_state.select_previous();
    }

    fn get_result(&mut self) -> Option<&'a str> {
        self.confirmed
            .then(|| self.list_state.selected())
            .flatten()
            .map(|index| self.filtered_options[index])
    }

    pub fn draw(&mut self, frame: &mut Frame) {
        // layout
        let layout =
            Layout::vertical([Constraint::Length(3), Constraint::Min(1)]).split(frame.area());

        // input
        let input = Paragraph::new(self.input.as_str())
            .block(Block::default().title("Input").borders(Borders::ALL));
        frame.render_widget(input, layout[0]);

        // options
        let filtered_items: Vec<_> = self
            .filtered_options
            .iter()
            .map(|&option| ListItem::new(option).style(Style::default().fg(Color::White)))
            .collect();
        let list = List::new(filtered_items)
            .block(Block::default().title("Items").borders(Borders::ALL))
            .highlight_style(
                Style::default()
                    .bg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            );

        frame.render_stateful_widget(list, layout[1], &mut self.list_state);
    }

    // TODO: vim input
    fn handle_event(&mut self, event: KeyEvent) {
        if event.kind != KeyEventKind::Press {
            return;
        }

        match event.code {
            KeyCode::Esc => {
                self.running = false;
                self.confirmed = false;
            }
            KeyCode::Enter => {
                self.running = false;
                self.confirmed = true;
            }
            KeyCode::Char('j') if event.modifiers.contains(KeyModifiers::CONTROL) => {
                self.select_next()
            }
            KeyCode::Char('k') if event.modifiers.contains(KeyModifiers::CONTROL) => {
                self.select_previous()
            }
            KeyCode::Char(c) => {
                // Handle typing
                if c.is_ascii_graphic() || c == ' ' {
                    self.input.push(c);
                    self.filter_options();
                }
            }
            KeyCode::Backspace => {
                self.input.pop();
                self.filter_options();
            }
            _ => {} // Ignore other keys
        }
    }
}
