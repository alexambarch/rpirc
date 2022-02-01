use crate::ui::util::Route;

pub struct App {
    pub current_route: Route,
    pub input: Vec<char>,
    pub input_loc: usize,
    pub input_cursor_position: usize,
    pub history: Vec<String>,
    pub history_loc: usize,
    pub history_count: usize,
}

impl Default for App {
    fn default() -> App {
        App {
            current_route: Route::Startup,
            input: vec![],
            input_loc: 0,
            input_cursor_position: 0,
            history: vec![],
            history_loc: 0,
            history_count: 0,
        }
    }
}
