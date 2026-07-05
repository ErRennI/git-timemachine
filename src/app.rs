use crate::{
    git::{CommitInfo, DiffLine, GitManager},
    my_errors::TimeMachineError,
};
use git2::Oid;
use ratatui::widgets::ListState;
use std::collections::HashMap;

#[derive(PartialEq)]
pub enum ActivePanel {
    CommitListPanel,
    DiffDetailPanel,
}

pub struct App {
    pub commits: Vec<CommitInfo>,
    pub list_state: ListState,
    pub should_quit: bool,

    pub diff_cache: HashMap<Oid, Vec<DiffLine>>,
    pub current_diff: Vec<DiffLine>,

    pub active_panel: ActivePanel,
    pub diff_scroll: u16,
    pub diff_horizontal_scroll: u16,
    pub diff_max_width: u16,
}

impl App {
    pub fn new(git_manager: &GitManager) -> Result<Self, TimeMachineError> {
        let commits = git_manager.get_commits()?;

        if commits.is_empty() {
            return Err(TimeMachineError::EmptyRepository);
        }

        let mut list_state = ListState::default();
        list_state.select(Some(0));

        let mut app = App {
            commits,
            list_state,
            should_quit: false,
            diff_cache: HashMap::new(),
            current_diff: Vec::new(),
            active_panel: ActivePanel::CommitListPanel,
            diff_scroll: 0,
            diff_horizontal_scroll: 0,
            diff_max_width: 0,
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
                self.diff_scroll = 0;
                self.diff_horizontal_scroll = 0;
            }
        }
    }

    pub fn previous_commit(&mut self, git_manager: &GitManager) {
        if let Some(selected) = self.list_state.selected() {
            if selected > 0 {
                self.list_state.select(Some(selected - 1));
                let _ = self.update_curr_diff(git_manager);
                self.diff_scroll = 0;
                self.diff_horizontal_scroll = 0;
            }
        }
    }

    pub fn update_curr_diff(&mut self, git_manager: &GitManager) -> Result<(), TimeMachineError> {
        self.diff_scroll = 0;

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
        self.diff_max_width = self
            .current_diff
            .iter()
            .map(|l| l.content.len())
            .max()
            .unwrap_or(80) as u16;
        Ok(())
    }

    pub fn toggle_panel(&mut self) {
        self.active_panel = match self.active_panel {
            ActivePanel::DiffDetailPanel => ActivePanel::CommitListPanel,
            ActivePanel::CommitListPanel => ActivePanel::DiffDetailPanel,
        };
    }
}
