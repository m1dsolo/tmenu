use crate::filterer::{ContainsFilterer, Filterer, FuzzyFilterer};
use anyhow::Result;
use ratatui::{
    backend::CrosstermBackend,
    crossterm::event::{self, read, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame, Terminal,
};
use std::fs::File;

pub struct App<'a> {
    running: bool,
    query: String,
    cursor_pos: usize,
    result: Option<String>,
    filtered_options: Vec<&'a str>,
    matched_indices: Vec<Vec<usize>>,
    list_state: ListState,
    contains_filterer: ContainsFilterer<'a>,
    fuzzy_filterer: Option<FuzzyFilterer<'a>>,
}

impl<'a> App<'a> {
    pub fn new(options: &'a [String], use_fuzzy: bool) -> Self {
        let options = options
            .iter()
            .map(|option| option.as_str())
            .collect::<Vec<&'a str>>();

        let fuzzy_filterer = if use_fuzzy {
            Some(FuzzyFilterer::new(options.clone()))
        } else {
            None
        };

        Self {
            running: true,
            query: String::new(),
            cursor_pos: 0,
            result: None,
            filtered_options: options.clone(),
            matched_indices: vec![],
            list_state: ListState::default(),
            contains_filterer: ContainsFilterer::new(options),
            fuzzy_filterer,
        }
    }

    pub fn run(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<File>>,
    ) -> Result<Option<String>> {
        self.list_state
            .select(self.filtered_options.first().map(|_| 0));

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
        if let Some(ref fuzzy) = self.fuzzy_filterer {
            let results = fuzzy.filter_with_matches(&self.query);
            self.filtered_options = results.iter().map(|r| r.text).collect();
            self.matched_indices = results.into_iter().map(|r| r.matched_indices).collect();
        } else {
            self.filtered_options = self.contains_filterer.filter(&self.query);

            if !self.query.is_empty() {
                self.matched_indices = self
                    .filtered_options
                    .iter()
                    .map(|option| {
                        let mut indices = Vec::new();
                        let query_lower = self.query.to_lowercase();
                        let option_lower = option.to_lowercase();
                        let mut search_start = 0;

                        for pc in query_lower.chars() {
                            if let Some(pos) = option_lower[search_start..].find(pc) {
                                indices.push(search_start + pos);
                                search_start += pos + 1;
                            }
                        }
                        indices
                    })
                    .collect();
            } else {
                self.matched_indices = vec![vec![]; self.filtered_options.len()];
            }
        }

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
        let layout =
            Layout::vertical([Constraint::Length(3), Constraint::Min(1)]).split(frame.area());

        let query_area = layout[0];
        let inner_area = Block::default()
            .title("Input")
            .borders(Borders::ALL)
            .inner(query_area);

        let cursor_x = self.cursor_pos as u16;
        if cursor_x < inner_area.width {
            frame.set_cursor_position((inner_area.x + cursor_x, inner_area.y));
        }

        let query = Paragraph::new(self.query.as_str())
            .block(Block::default().title("Input").borders(Borders::ALL));
        frame.render_widget(query, query_area);

        let filtered_items: Vec<_> = self
            .filtered_options
            .iter()
            .enumerate()
            .map(|(idx, &option)| {
                if !self.query.is_empty() {
                    let indices = self.matched_indices.get(idx).cloned().unwrap_or_default();
                    if indices.is_empty() {
                        ListItem::new(option).style(Style::default().fg(Color::White))
                    } else {
                        let chars: Vec<char> = option.chars().collect();
                        let mut spans = Vec::new();
                        let mut last_idx = 0;

                        for &idx in &indices {
                            if idx > last_idx {
                                spans.push(Span::raw(
                                    chars[last_idx..idx].iter().collect::<String>(),
                                ));
                            }
                            spans.push(Span::raw(chars[idx].to_string()).style(
                                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                            ));
                            last_idx = idx + 1;
                        }

                        if last_idx < chars.len() {
                            spans.push(Span::raw(chars[last_idx..].iter().collect::<String>()));
                        }

                        ListItem::new(Line::from(spans))
                    }
                } else {
                    ListItem::new(option).style(Style::default().fg(Color::White))
                }
            })
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

    fn move_cursor_left(&mut self) {
        if self.cursor_pos > 0 {
            self.cursor_pos -= 1;
        }
    }

    fn move_cursor_right(&mut self) {
        if self.cursor_pos < self.query.len() {
            self.cursor_pos += 1;
        }
    }

    fn move_cursor_to_start(&mut self) {
        self.cursor_pos = 0;
    }

    fn move_cursor_to_end(&mut self) {
        self.cursor_pos = self.query.len();
    }

    fn delete_char_before_cursor(&mut self) {
        if self.cursor_pos > 0 {
            self.query.remove(self.cursor_pos - 1);
            self.cursor_pos -= 1;
            self.filter_options();
        }
    }

    fn delete_word_before_cursor(&mut self) {
        if self.cursor_pos > 0 {
            let mut pos = self.cursor_pos;
            while pos > 0 && self.query.chars().nth(pos - 1) == Some(' ') {
                pos -= 1;
            }
            while pos > 0 && self.query.chars().nth(pos - 1) != Some(' ') {
                pos -= 1;
            }
            self.query.drain(pos..self.cursor_pos);
            self.cursor_pos = pos;
            self.filter_options();
        }
    }

    fn clear_query(&mut self) {
        self.query.clear();
        self.cursor_pos = 0;
        self.filter_options();
    }

    fn insert_char(&mut self, c: char) {
        self.query.insert(self.cursor_pos, c);
        self.cursor_pos += 1;
        self.filter_options();
    }

    // TODO: vim keybindings
    fn handle_event(&mut self, event: KeyEvent) {
        if event.kind != KeyEventKind::Press {
            return;
        }

        match (event.modifiers, event.code) {
            (_, KeyCode::Esc) => {
                self.running = false;
                self.result = None;
            }
            (_, KeyCode::Enter) => {
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
            (_, KeyCode::Down)
            | (_, KeyCode::Tab)
            | (KeyModifiers::CONTROL, KeyCode::Char('j')) => self.select_next(),
            (_, KeyCode::Up)
            | (_, KeyCode::BackTab)
            | (KeyModifiers::CONTROL, KeyCode::Char('k')) => self.select_previous(),
            (_, KeyCode::Left) => self.move_cursor_left(),
            (_, KeyCode::Right) => self.move_cursor_right(),
            (_, KeyCode::Home) | (KeyModifiers::CONTROL, KeyCode::Char('a')) => {
                self.move_cursor_to_start()
            }
            (_, KeyCode::End) | (KeyModifiers::CONTROL, KeyCode::Char('e')) => {
                self.move_cursor_to_end()
            }
            (_, KeyCode::Backspace) => self.delete_char_before_cursor(),
            (KeyModifiers::CONTROL, KeyCode::Char('w')) => self.delete_word_before_cursor(),
            (KeyModifiers::CONTROL, KeyCode::Char('u')) => self.clear_query(),
            (_, KeyCode::Char(c)) => {
                if c.is_ascii_graphic() || c == ' ' {
                    self.insert_char(c);
                }
            }
            _ => {}
        }
    }
}
