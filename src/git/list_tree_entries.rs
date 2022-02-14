use std::path;

use matches;
use git2;

#[derive(Debug)]
pub enum FileListingError {
    CannotGetRoot,
    CannotGetDirectory,
    CannotGetDirectoryObject,
    ProjectDirectoryIsNotATree,
}

pub fn list_files<'tree, 'repository>(tree: &'tree git2::Tree<'repository>)
-> impl Iterator<Item = git2::TreeEntry<'tree>> {
    tree.iter().filter(|entry| entry.kind().filter(|kind| matches!(kind, git2::ObjectType::Blob)).is_some())
}

pub fn get_tree<'repository>(repository: &'repository git2::Repository, commit: git2::Commit<'repository>, path: &path::Path)
-> Result<git2::Tree<'repository>, FileListingError> {
    let root = commit
    .tree()
    .or(Err(FileListingError::CannotGetRoot))?;

    let directory = root.get_path(path).or(Err(FileListingError::CannotGetDirectory))?;

    let directory_object = directory
    .to_object(repository)
    .or(Err(FileListingError::CannotGetDirectoryObject))?;

    directory_object
    .into_tree()
    .or(Err(FileListingError::ProjectDirectoryIsNotATree))
}
