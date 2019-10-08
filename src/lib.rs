extern crate git2;

#[derive(Debug)]
pub enum Status {
    Clean,
    Dirty,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Status::Clean => "clean",
            Status::Dirty => "dirty",
        };

        write!(f, "{}", s)
    }
}

pub struct Repository {
    repo: git2::Repository,
}

impl Repository {
    pub fn new(path: Option<&std::path::Path>) -> Result<Repository, git2::Error> {
        let path = match path {
            Some(p) => p,
            None => std::path::Path::new("."),
        };

        Ok(Repository {
            repo: git2::Repository::discover(path)?,
        })
    }

    pub fn commit_short_id(&self) -> Result<String, git2::Error> {
        let id = self.repo.head()?.peel_to_commit()?.as_object().short_id()?;
        String::from_utf8(id.to_vec()).map_err(|_| git2::Error::from_str(""))
    }

    pub fn commit_id(&self) -> Result<String, git2::Error> {
        Ok(self
            .repo
            .head()?
            .peel_to_commit()?
            .as_object()
            .id()
            .to_string())
    }

    pub fn status(&self) -> Result<Status, git2::Error> {
        let mut repo_opts = git2::StatusOptions::new();
        repo_opts.include_ignored(false);
        repo_opts.include_untracked(true);
        repo_opts.show(git2::StatusShow::Workdir);

        Ok(if self.repo.statuses(Some(&mut repo_opts))?.is_empty() {
            Status::Clean
        } else {
            Status::Dirty
        })
    }
}
