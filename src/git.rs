use git2::{Oid, Repository, Sort};

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
    pub fn new() -> Result<Self, git2::Error> {
        let repo = Repository::open(".")?;
        Ok(GitManager { repo })
    }

    pub fn get_commits(&self) -> Result<Vec<CommitInfo>, git2::Error> {
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
}
