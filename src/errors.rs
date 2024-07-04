use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct DirectoryNotRootError;


impl Display for DirectoryNotRootError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Current working directory is not root of a Flutter Project!")
    }
}

impl Error for DirectoryNotRootError {}
