use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use anyhow::Result;
use tokio::sync::mpsc::Receiver;
use std::io::stdout;
use std::io::Stdout;
use tui::{Terminal, Frame,
           backend::{Backend, CrosstermBackend},
           widgets::{Paragraph, Block, Borders, List, ListItem},
           layout::{Layout, Constraint, Direction, Rect},
           text::Span,
           style::{Style, Modifier},
};
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen}
};
use util::{UiEvent, Route};
use unicode_width::UnicodeWidthChar;

use crate::app::App;


pub struct Ui<B>
where B: Backend
{
    term: Terminal<B>,
    current_tab: usize,
    tab_count: usize,
}

impl Default for Ui<CrosstermBackend<Stdout>>
{
    fn default() -> Self {
        enable_raw_mode().unwrap();

        match execute!(stdout(), EnterAlternateScreen) {
            Ok(a) => a,
            _ => unreachable!(),
        };

        let mut term = Terminal::new(CrosstermBackend::new(stdout())).unwrap();
        term.clear().unwrap();

        Ui {
            term,
            current_tab: 0,
            tab_count: 1,
        }
    }
}

impl Ui<CrosstermBackend<Stdout>> {
    pub fn new() -> Ui<CrosstermBackend<Stdout>> {
        Default::default()
    }

    pub async fn listen(&mut self,
                             mut rx: Receiver<util::UiEvent>,
                             app: &mut App) {
        while let Some(event) = rx.recv().await {
            match event {
                UiEvent::Buffer(ch) => {
                    app.input.insert(app.input_loc, ch);
                    app.input_loc += 1;
                    app.input_cursor_position += UnicodeWidthChar::width(ch).unwrap();
                }

                UiEvent::Del => {
                    let ch = app.input.remove(app.input_loc);
                    app.input_loc -= 1;
                    app.input_cursor_position -= UnicodeWidthChar::width(ch).unwrap();
                }

                UiEvent::Left => {
                    app.input_loc -= 1;
                    app.input_cursor_position -= UnicodeWidthChar::width(*app.input.get(app.input_loc).unwrap()).unwrap();
                }

                UiEvent::Right => {
                    app.input_cursor_position += UnicodeWidthChar::width(*app.input.get(app.input_loc).unwrap()).unwrap();
                    app.input_loc += 1;
                }

                UiEvent::Terminate => {
                    disable_raw_mode().unwrap();
                    execute!(stdout(), LeaveAlternateScreen).unwrap();
                    break;
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
                            ].as_ref()
                        )
                        .split(f.size());

                    let header_text = Span::raw("rpirc -- version 0.1.0");
                    let header = Paragraph::new(header_text)
                        .block(Block::default()
                               .title("Welcome!")
                               .borders(Borders::ALL));
                    f.render_widget(header, chunks[0]);

                    let items = [ListItem::new("irc.freenode.net"),
                                 ListItem::new("irc.orpheus.network")];
                    let body = List::new(items)
                        .block(Block::default()
                               .title("Quickconnect")
                               .borders(Borders::NONE))
                        .highlight_style(Style::default()
                                         .add_modifier(Modifier::BOLD))
                        .highlight_symbol(">>");
                    f.render_widget(body, chunks[1]);

                    draw_input_box(f, input.iter().collect(), chunks[2]);
                }) {
                    Ok(a) => a,
                    _ => unreachable!(),
                };
            }

            Route::Channel => {
                // TODO
            }

            Route::PrivMsg => {
                // TODO
            }
        }
    }
}

fn draw_input_box<B>(f: &mut Frame<B>,
                     input: String,
                     layout_chunk: Rect)
where B: Backend
{
    let text = Span::raw(input);
    let block = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL));
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
        Right
    }

    // Routes in the app
    pub enum Route {
        Startup,
        Channel,
        PrivMsg,
    }

    // Actions on Tabs
    pub enum TabEvent {
        Create,
        Delete(usize),
        View(usize)
    }
}
