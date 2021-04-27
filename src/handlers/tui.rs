use crate::ui::util::{Route};
use crate::event::InputBuffer;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io::stdout;
use std::io::Stdout;
use tui::{ Terminal, Frame,
           backend::{Backend, CrosstermBackend},
           widgets::{Paragraph, Block, Borders, List, ListItem},
           layout::{Layout, Constraint, Direction, Rect},
           text::Span,
           style::{Style, Modifier},
};
use futures::{future::FutureExt, StreamExt};
use crossterm::{
    event::{Event, EventStream, KeyCode, KeyEvent, KeyModifiers},
};

pub fn stop_ui() {
    disable_raw_mode();
}

pub struct Ui<B>
where B: Backend
{
    term: Terminal<B>,
    current_tab: usize,
    tab_count: usize,
    current_route: Route,
    ibuf: InputBuffer,
}


impl Default for Ui<CrosstermBackend<Stdout>>
{
    fn default() -> Self {
        enable_raw_mode();
        let term = Terminal::new(CrosstermBackend::new(stdout())).unwrap();
        Ui {
            term,
            current_tab: 0,
            tab_count: 1,
            current_route: Route::Startup,
            ibuf: InputBuffer::new(),
        }
    }
}

impl Ui<CrosstermBackend<Stdout>> {
    pub fn new() -> Ui<CrosstermBackend<Stdout>> {
        Default::default()
    }

    pub async fn listen(&mut self) {
        let mut reader = EventStream::new();

        while let Some(event) = reader.next().fuse().await {
            match event.unwrap() {
                // Send character keypresses to text input handler
                Event::Key(KeyEvent{code: KeyCode::Char(ch),
                                    modifiers: KeyModifiers::NONE}) => {
                    self.ibuf.message_buffer.push(ch);
                }

                // Enter key executes whatever is inside of the input buffer.
                Event::Key(KeyEvent{code: KeyCode::Enter,
                                    modifiers: KeyModifiers::NONE}) => {
                    self.ibuf.execute().unwrap();
                }

                // Keyboard Interrupt
                Event::Key(KeyEvent{code: KeyCode::Char('c'),
                                    modifiers: KeyModifiers::CONTROL}) => {
                    println!("Caught keyboard interrupt.");
                    break;
                }

                // Some other case I'm sure I'm forgetting
                _ => {
                    panic!("Unhandled terminal event");
                }
            }
        }
    }

    pub fn draw(&mut self, route: Route) {
        match route {
            Route::Startup => {
                self.term.draw(|f| {
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
                        .block(Block::default().title("Welcome!").borders(Borders::ALL));
                    f.render_widget(header, chunks[0]);

                    let items = [ListItem::new("irc.freenode.net"),
                                 ListItem::new("irc.orpheus.network")];
                    let body = List::new(items)
                        .block(Block::default().title("Quickconnect").borders(Borders::NONE))
                        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                        .highlight_symbol(">>");
                    f.render_widget(body, chunks[1]);

                    draw_input_box(f, &self.ibuf, chunks[2]);
                });
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
                     input: &InputBuffer,
                     layout_chunk: Rect)
where B: Backend
{
    let text = Span::raw(input.message_buffer.clone());
    let block = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(block, layout_chunk);
}
