use std::path::{Path, PathBuf};
use thiserror::Error;

pub fn validate_worlds(worlds: &Vec<PathBuf>) -> Result<(), ValidateWorldError> {
    for world in worlds {
        validate_world(&world)?;
    }
    Ok(())
}

fn validate_world(world_dir: &Path) -> Result<(), ValidateWorldError> {
    if !world_dir.is_dir() {
        return Err(ValidateWorldError::NotADirectory);
    }

    if !std::fs::exists(world_dir.join("level.dat")).unwrap_or(false) {
        return Err(ValidateWorldError::MissingLevelData);
    }

    Ok(())
}

#[derive(Error, Debug)]
pub enum ValidateWorldError {
    #[error("the provided world directory is not a directory")]
    NotADirectory,
    #[error("the provided world directory is missing the `level.dat` file")]
    MissingLevelData,
}
