use std::sync;

use git2;
use antidote;

use crate::phases::global;

unsafe impl Send for CommandInput
{}

pub struct CommandInput {
    pub global:     sync::Arc<antidote::RwLock<global::Global>>,
    pub repository: sync::Arc<antidote::RwLock<git2::Repository>>,
}

impl CommandInput {
    pub fn read(&self) -> (antidote::RwLockReadGuard<global::Global>, antidote::RwLockReadGuard<git2::Repository>) {
        (self.global.read(), self.repository.read())
    }
}

impl From<(sync::Arc<antidote::RwLock<global::Global>>, sync::Arc<antidote::RwLock<git2::Repository>>)> for CommandInput {
    fn from((global, repository): (sync::Arc<antidote::RwLock<global::Global>>, sync::Arc<antidote::RwLock<git2::Repository>>)) -> Self {
        CommandInput { global, repository }
    }
}
