use crate::git::CommitInfo;
use ratatui::widgets::ListState;
use std::error::Error;

pub struct App {
    pub commits: Vec<CommitInfo>,
    pub list_state: ListState,
    pub should_quit: bool,
}

impl App {
    //pub fn new() -> Result<Self, >
}
