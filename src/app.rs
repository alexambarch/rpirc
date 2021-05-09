use crate::ui::util::Route;

pub struct App {
    pub current_route: Route,
    pub input: Vec<char>,
    pub input_loc: usize,
    pub input_cursor_position: usize,
}

impl Default for App {
    fn default() -> App {
        App {
            current_route: Route::Startup,
            input: vec![],
            input_loc: 0,
            input_cursor_position: 0
        }
    }
}
