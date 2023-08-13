use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("File read error for '{file}' | {error}")]
    FileReadError { error: io::Error, file: PathBuf },

    #[error("File write error for '{file}' | {error}")]
    FileWriteError { error: io::Error, file: PathBuf },

    #[error("Error while decoding '{file}' as JSON | {error}")]
    JSONDecodingError {
        error: serde_json::Error,
        file: PathBuf,
    },

    #[error("Error while encoding JSON into '{file}' | {error}")]
    JSONEncodingError {
        error: serde_json::Error,
        file: PathBuf,
    },
}

pub fn load_json<T: for<'a> Deserialize<'a>>(path: &Path, default: T) -> Result<T, StorageError> {
    match fs::read_to_string(path) {
        Ok(json_string) => serde_json::from_str(json_string.as_str()).map_err(|error| {
            StorageError::JSONDecodingError {
                error,
                file: path.to_path_buf(),
            }
        }),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(default),
        Err(error) => Err(StorageError::FileReadError {
            error,
            file: path.to_path_buf(),
        }),
    }
}

pub fn save_json<T: Serialize>(path: &Path, value: &T) -> Result<(), StorageError> {
    let json =
        serde_json::to_string_pretty(value).map_err(|error| StorageError::JSONEncodingError {
            error,
            file: path.to_path_buf(),
        })?;
    fs::write(path, json).map_err(|error| StorageError::FileWriteError {
        error,
        file: path.to_path_buf(),
    })?;
    Ok(())
}
