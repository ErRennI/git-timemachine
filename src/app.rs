use crate::{
    git::{self, CommitInfo, GitManager},
    my_errors::TimeMachineError,
};
use ratatui::widgets::ListState;

pub struct App {
    pub commits: Vec<CommitInfo>,
    pub list_state: ListState,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Result<Self, TimeMachineError> {
        let git_manager = GitManager::new()?;
        let commits = git_manager.get_commits()?;

        let mut list_state = ListState::default();
        if !commits.is_empty() {
            list_state.select(Some(0));
        }

        Ok(App {
            commits,
            list_state,
            should_quit: false,
        })
    }

    pub fn next_commit(&mut self) {
        if let Some(selected) = self.list_state.selected() {
            if selected < self.commits.len() - 1 {
                self.list_state.select(Some(selected + 1));
            }
        }
    }

    pub fn previous_commit(&mut self) {
        if let Some(selected) = self.list_state.selected() {
            if selected > 0 {
                self.list_state.select(Some(selected - 1));
            }
        }
    }
}
