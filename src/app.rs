use anyhow::Result;
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, read},
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};
use std::fs::File;

pub struct App<'a> {
    running: bool,
    query: String,
    result: Option<String>,
    all_options: Vec<&'a str>,
    filtered_options: Vec<&'a str>,
    list_state: ListState,
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
            query: String::new(),
            result: None,
            all_options: options_refs.clone(),
            filtered_options: options_refs,
            list_state,
        }
    }

    pub fn run(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<File>>,
    ) -> Result<Option<String>> {
        while self.running {
            terminal.draw(|frame| self.draw(frame))?;
            if event::poll(std::time::Duration::from_millis(50))? {
                if let Event::Key(key_event) = read()? {
                    self.handle_event(key_event);
                }
            }
        }

        Ok(self.result.take())
    }

    fn filter_options(&mut self) {
        let query_lower = self.query.to_lowercase();
        self.filtered_options = self
            .all_options
            .iter()
            .filter(|option| option.to_lowercase().contains(&query_lower))
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

    pub fn draw(&mut self, frame: &mut Frame) {
        // layout
        let layout =
            Layout::vertical([Constraint::Length(3), Constraint::Min(1)]).split(frame.area());

        // query
        let query = Paragraph::new(self.query.as_str())
            .block(Block::default().title("Input").borders(Borders::ALL));
        frame.render_widget(query, layout[0]);

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

    // TODO: vim keybindings
    fn handle_event(&mut self, event: KeyEvent) {
        if event.kind != KeyEventKind::Press {
            return;
        }

        match event.code {
            KeyCode::Esc => {
                self.running = false;
                self.result = None;
            }
            KeyCode::Enter => {
                self.running = false;

                if event.modifiers.contains(KeyModifiers::ALT) {
                    self.result = Some(self.query.clone());
                } else {
                    self.result = self
                        .list_state
                        .selected()
                        .and_then(|index| self.filtered_options.get(index))
                        .map(|&option| option.to_string())
                        .or_else(|| Some(self.query.clone()));
                }
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
                    self.query.push(c);
                    self.filter_options();
                }
            }
            KeyCode::Backspace => {
                self.query.pop();
                self.filter_options();
            }
            _ => {} // Ignore other keys
        }
    }
}
