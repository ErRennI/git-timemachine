use crate::TimeMachineError;
use git2::{Oid, Repository, Sort};

#[derive(Clone)]
pub struct DiffLine {
    pub content: String,
    pub line_type: char,
    pub file_name: String,
}
pub struct CommitInfo {
    pub id: Oid,
    pub short_id: String,
    pub summary: String,
    pub author: String,
}

pub struct GitManager {
    repo: Repository,
}

//TODO: In the later stages it should take an directory argument
impl GitManager {
    pub fn new(repo_path: &str) -> Result<Self, TimeMachineError> {
        let repo = Repository::open(repo_path)?;
        Ok(GitManager { repo })
    }

    pub fn get_commits(&self) -> Result<Vec<CommitInfo>, TimeMachineError> {
        let mut revwalk = self.repo.revwalk()?;
        revwalk.push_head()?;
        revwalk.set_sorting(Sort::TIME)?;

        let mut commits: Vec<CommitInfo> = Vec::new();

        for id_result in revwalk {
            if let Ok(commit_id) = id_result {
                if let Ok(commit) = self.repo.find_commit(commit_id) {
                    commits.push(CommitInfo {
                        id: commit_id,
                        short_id: commit_id.to_string()[..7].to_string(),
                        summary: commit.summary().unwrap_or("").to_string(),
                        author: commit.author().name().unwrap_or("").to_string(),
                    });
                }
            }
        }
        Ok(commits)
    }

    pub fn get_commit_diff(&self, commit_id: Oid) -> Result<Vec<DiffLine>, TimeMachineError> {
        let commit = self.repo.find_commit(commit_id)?;
        let commit_tree = commit.tree()?;

        let parent_tree = if commit.parent_count() > 0 {
            Some(commit.parent(0)?.tree()?)
        } else {
            None
        };

        let diff = self
            .repo
            .diff_tree_to_tree(parent_tree.as_ref(), Some(&commit_tree), None)?;

        let mut diff_lines: Vec<DiffLine> = Vec::new();

        diff.print(git2::DiffFormat::Patch, |delta, _hunk, line| {
            let origin = line.origin();
            let content = String::from_utf8_lossy(line.content()).into_owned();

            let file_name = delta
                .new_file()
                .path()
                .and_then(|p| p.to_str())
                .unwrap_or("Unknown file")
                .to_string();

            diff_lines.push(DiffLine {
                content,
                line_type: origin,
                file_name,
            });

            true
        })?;

        Ok(diff_lines)
    }
}
