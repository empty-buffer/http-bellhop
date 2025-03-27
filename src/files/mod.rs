use crate::tools::files::configs::FilesError::FileNotFound;
use std::path::PathBuf;
use std::{
    fs::{self},
    io,
    path::Path,
};

pub type Result<T> = core::result::Result<T, Error>;

pub enum Error {
    FolderNotFound,
    FileNotFound,
    Error(String),
}

impl From<io::Error> for FilesError {
    fn from(err: io::Error) -> Self {
        Self::Error(err.to_string())
    }
}

pub fn get_config(path: &str) -> Result<PathBuf, Error> {
    let file = PathBuf::from(path);
    if file.is_file() {
        return Ok(file);
    }

    if file.is_dir() {
        return Ok(file);
    }

    Err(FileNotFound)
}

pub fn get_files(dir: &Path, files: &mut Vec<PathBuf>) -> Result<(), FilesError> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                get_files(&path, files)?;
            } else {
                files.push(entry.path());
                // println!("{:?}", entry.path()) // uncomment to check all files
            }
        }
    }
    Ok(())
}

pub fn get_input() -> Option<usize> {
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Please pick one of displayed options");
    }
    let input = buffer.trim().to_lowercase().to_owned();
    if input.is_empty() {
        None
    } else {
        Some(input.parse::<usize>().expect(""))
    }
}
