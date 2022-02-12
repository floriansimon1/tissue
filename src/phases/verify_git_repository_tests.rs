use std::fs;
use tempfile;

use crate::phases::verify_git_repository;
use crate::logging::logger;

#[test]
fn check_git_repository_discovery_in_actual_git_folder() {
    let directory = tempfile::tempdir().unwrap();

    let path      = directory.path();

    let _         = git2::Repository::init(&path);

    let discovered_path = verify_git_repository
    ::open_repository::<()>(&logger::Logger::new().unwrap(), path)
    .map(|repository| String::from(repository.path().parent().unwrap().to_str().unwrap()))
    .unwrap_or(String::from("<Could not find a path>"));

    assert_eq!(discovered_path, path.to_str().unwrap());
}

#[test]
fn check_git_repository_discovery_in_child_of_git_folder() {
    let directory  = tempfile::tempdir().unwrap();

    let path       = directory.path();

    let child_path = directory.path().join("a").join("b").join("c");

    fs::create_dir_all(&child_path).unwrap();

    let _          = git2::Repository::init(&path);

    let discovered_path = verify_git_repository
    ::open_repository::<()>(&logger::Logger::new().unwrap(), &child_path)
    .map(|repository| String::from(repository.path().parent().unwrap().to_str().unwrap()))
    .unwrap_or(String::from("<Could not find a path>"));

    assert_eq!(discovered_path, path.to_str().unwrap());
}

#[test]
fn check_git_repository_discovery_in_non_git_folder() {
    let directory = tempfile::tempdir().unwrap();

    let path      = directory.path();

    let failed    = verify_git_repository::open_repository::<()>(&logger::Logger::new().unwrap(), path).is_err();

    assert!(failed, "Repository detection in a non-git folder did not fail as expected!");
}
