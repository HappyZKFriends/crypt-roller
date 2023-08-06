use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug)]
pub enum StorageError {
    FileReadError {
        error: io::Error,
        file: PathBuf,
    },
    FileWriteError {
        error: io::Error,
        file: PathBuf,
    },
    JSONDecodingError {
        error: serde_json::Error,
        file: PathBuf,
    },
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
