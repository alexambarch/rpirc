use crossterm::execute;
use crossterm::terminal::EnterAlternateScreen;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io::stdout;
use std::io::Stdout;
use tokio::sync::mpsc::Receiver;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::Span,
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};

use unicode_width::UnicodeWidthChar;
use util::{Route, UiEvent};

use crate::app::App;

pub struct Ui<B>
where
    B: Backend,
{
    term: Terminal<B>,
}

impl Default for Ui<CrosstermBackend<Stdout>> {
    fn default() -> Self {
        enable_raw_mode().unwrap();

        match execute!(stdout(), EnterAlternateScreen) {
            Ok(a) => a,
            _ => unreachable!(),
        };

        let mut term = Terminal::new(CrosstermBackend::new(stdout())).unwrap();
        term.clear().unwrap();

        Ui { term }
    }
}

impl Ui<CrosstermBackend<Stdout>> {
    pub fn new() -> Ui<CrosstermBackend<Stdout>> {
        Default::default()
    }

    pub async fn listen(&mut self, mut rx: Receiver<util::UiEvent>, app: &mut App) {
        self.draw(&Route::Startup, &app);
        while let Some(event) = rx.recv().await {
            match event {
                UiEvent::Buffer(ch) => {
                    app.input.insert(app.input_loc, ch);
                    app.input_loc += 1;
                    app.input_cursor_position += UnicodeWidthChar::width(ch).unwrap();
                }

                UiEvent::Del if app.input_loc > 0 => {
                    let ch = app.input.remove(app.input_loc - 1);
                    app.input_loc -= 1;
                    app.input_cursor_position -= UnicodeWidthChar::width(ch).unwrap();
                }

                UiEvent::Left if app.input_loc > 0 => {
                    app.input_loc -= 1;
                    app.input_cursor_position -=
                        UnicodeWidthChar::width(*app.input.get(app.input_loc).unwrap()).unwrap();
                }

                UiEvent::Right if app.input_loc < app.input.len() => {
                    app.input_cursor_position +=
                        UnicodeWidthChar::width(*app.input.get(app.input_loc).unwrap()).unwrap();
                    app.input_loc += 1;
                }

                UiEvent::Up => {
                    app.history_loc = app.history_loc + 1;
                    app.input = app
                        .history
                        .get(new_history_loc)
                        .unwrap()
                        .chars()
                        .collect::<Vec<char>>();
                    app.history_loc = new_history_loc;
                }

                UiEvent::Down => {
                    if app.history_loc == 0 {
                        app.input = vec![];
                    }
                }

                UiEvent::Terminate => {
                    break;
                }

                UiEvent::Execute if app.input.len() > 0 => {
                    if handle_input(app.input.iter().collect()).is_none() {
                        disable_raw_mode().unwrap();
                        break;
                    }
                }

                UiEvent::Scene(sc) => {
                    app.current_route = sc;
                }

                UiEvent::Tab(ev) => {
                    handle_tab_event(ev, app);
                }

                _ => {}
            }

            self.draw(&app.current_route, app);
        }

        rx.close();
    }

    pub fn draw(&mut self, route: &Route, app: &App) {
        let input = &app.input;

        match route {
            Route::Startup => {
                match self.term.draw(|f| {
                    let chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .margin(1)
                        .constraints(
                            [
                                Constraint::Percentage(10),
                                Constraint::Percentage(80),
                                Constraint::Percentage(10),
                            ]
                            .as_ref(),
                        )
                        .split(f.size());

                    let header_text = Span::raw("rpirc -- version 0.1.0");
                    let header = Paragraph::new(header_text)
                        .block(Block::default().title("Welcome!").borders(Borders::ALL));
                    f.render_widget(header, chunks[0]);

                    let items = [
                        ListItem::new("irc.freenode.net"),
                        ListItem::new("irc.orpheus.network"),
                    ];
                    let body = List::new(items)
                        .block(
                            Block::default()
                                .title("Quickconnect")
                                .borders(Borders::NONE),
                        )
                        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                        .highlight_symbol(">>");
                    f.render_widget(body, chunks[1]);

                    draw_input_box(f, input.iter().collect(), chunks[2]);
                }) {
                    Ok(a) => a,
                    _ => unreachable!(),
                };
            }

            Route::Server => {
                todo!();
            }

            Route::Channel => {
                todo!();
            }

            Route::PrivMsg => {
                todo!();
            }
        }
    }
}

fn draw_input_box<B>(f: &mut Frame<B>, input: String, layout_chunk: Rect)
where
    B: Backend,
{
    let text = Span::raw(input);
    let block = Paragraph::new(text).block(Block::default().borders(Borders::ALL));
    f.render_widget(block, layout_chunk);
}

pub mod util {
    // Redraw the UI, Terminate the UI, or perform an action on tabs
    pub enum UiEvent {
        Buffer(char),
        Del,
        Execute,
        Scene(Route),
        Tab(TabEvent),
        Terminate,
        Left,
        Right,
        Up,
        Down,
    }

    // Routes in the app
    pub enum Route {
        Startup,
        Server,
        Channel,
        PrivMsg,
    }

    // Actions on Tabs
    pub enum TabEvent {
        Create,
        Delete(usize),
        View(usize),
    }
}

// Returns Some to run on input command, else None
fn handle_input(input: String) -> Option<()> {
    if input.starts_with('/') {
        let args: Vec<&str> = input[1..].split(' ').collect();

        match args[0] {
            "quit" => return None,
            "connect" => {}

            _ => {}
        }
    }

    Some(())
}

fn handle_tab_event(event: util::TabEvent, app: &mut App) {
    match event {
        _ => {}
    }
}
