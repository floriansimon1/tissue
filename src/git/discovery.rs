use std::path;

use git2;

pub fn discover_repository(path: &path::Path) -> Result<git2::Repository, git2::Error> {
    path
    .to_str()
    .filter(|path| *path == ".")
    .map(|_| git2::Repository::open_from_env())
    .unwrap_or_else(|| git2::Repository::discover(&path))
}
