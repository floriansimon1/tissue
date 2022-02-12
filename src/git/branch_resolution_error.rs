use std::fmt;

use git2;

pub enum BranchResolutionError {
    UnknownError(git2::Error),
    ReferenceHasNoObjectId,
    CommitNotFound,
}

impl fmt::Display for BranchResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let message = match self {
            BranchResolutionError::CommitNotFound         => String::from("commit not found"),
            BranchResolutionError::ReferenceHasNoObjectId => String::from("object ID not found"),
            BranchResolutionError::UnknownError(error)    => format!("unknown error: {}", error.message()),
        };

        write!(formatter, "Branch resolution failed ({})", message)
    }
}
