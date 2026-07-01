use crate::{
    git::{self, CommitInfo, DiffLine, GitManager},
    my_errors::{self, TimeMachineError},
};
use git2::Oid;
use ratatui::widgets::ListState;
use std::collections::HashMap;

pub struct App {
    pub commits: Vec<CommitInfo>,
    pub list_state: ListState,
    pub should_quit: bool,

    pub diff_chache: HashMap<Oid, Vec<DiffLine>>,
    pub current_diff: Vec<DiffLine>,
}

impl App {
    pub fn new() -> Result<Self, TimeMachineError> {
        let git_manager = GitManager::new()?;
        let commits = git_manager.get_commits()?;

        let mut list_state = ListState::default();
        if !commits.is_empty() {
            list_state.select(Some(0));
        }

        let mut app = App {
            commits,
            list_state,
            should_quit: false,
            diff_chache: HashMap::new(),
            current_diff: Vec::new(),
        };

        if !app.commits.is_empty() {
            app.update_curr_diff(git_manager)?;
        }

        Ok(app)
    }
    //TODO update_cureent_diffi tetikle
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

    pub fn update_curr_diff(&mut self, git_manager: GitManager) -> Result<(), TimeMachineError> {}
}
