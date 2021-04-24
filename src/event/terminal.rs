use crossterm::{
    event::{Event, EventStream, KeyCode},
};
use futures::{future::FutureExt, select, StreamExt};
