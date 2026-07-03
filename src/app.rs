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

    pub diff_cache: HashMap<Oid, Vec<DiffLine>>,
    pub current_diff: Vec<DiffLine>,
}

impl App {
    pub fn new(git_manager: &GitManager) -> Result<Self, TimeMachineError> {
        let commits = git_manager.get_commits()?;

        let mut list_state = ListState::default();
        if !commits.is_empty() {
            list_state.select(Some(0));
        }

        let mut app = App {
            commits,
            list_state,
            should_quit: false,
            diff_cache: HashMap::new(),
            current_diff: Vec::new(),
        };

        if !app.commits.is_empty() {
            app.update_curr_diff(&git_manager)?;
        }

        Ok(app)
    }

    pub fn next_commit(&mut self, git_manager: &GitManager) {
        if let Some(selected) = self.list_state.selected() {
            if selected < self.commits.len() - 1 {
                self.list_state.select(Some(selected + 1));
                let _ = self.update_curr_diff(git_manager);
            }
        }
    }

    pub fn previous_commit(&mut self, git_manager: &GitManager) {
        if let Some(selected) = self.list_state.selected() {
            if selected > 0 {
                self.list_state.select(Some(selected - 1));
                let _ = self.update_curr_diff(git_manager);
            }
        }
    }

    pub fn update_curr_diff(&mut self, git_manager: &GitManager) -> Result<(), TimeMachineError> {
        if let Some(selected) = self.list_state.selected() {
            if let Some(curr_commit) = self.commits.get(selected) {
                let curr_oid = curr_commit.id;

                if self.diff_cache.contains_key(&curr_oid) {
                    if let Some(cached_diff) = self.diff_cache.get(&curr_oid) {
                        self.current_diff = cached_diff.clone();
                    }
                } else {
                    let new_diff = git_manager.get_commit_diff(curr_oid)?;

                    self.diff_cache.insert(curr_oid, new_diff.clone());

                    self.current_diff = new_diff;
                }
            }
        } else {
            self.current_diff.clear();
        }
        Ok(())
    }
}
