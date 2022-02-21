
#[derive(Debug)]
pub enum TreeListingError {
    CannotGetRoot,
    TreeNotADirectory,
    CannotGetTreeEntry,
    CannotGetDirectoryObject,
}
