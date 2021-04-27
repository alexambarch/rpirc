// Redraw the UI, Terminate the UI, or perform an action on tabs
pub enum UiEvent {
    Redraw,
    Terminate,
    Scene(Route),
    Tab(TabEvent),
}

// Routes in the app
pub enum Route {
    Startup,
    Channel,
    PrivMsg,
}

// Actions on Tabs
enum TabEvent {
    Create,
    Delete(usize),
    View(usize)
}
