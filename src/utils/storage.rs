use std::fs;
use std::io;

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug)]
pub enum StorageError {
    FileReadError {
        error: io::Error,
        file: String,
    },
    FileWriteError {
        error: io::Error,
        file: String,
    },
    JSONDecodingError {
        error: serde_json::Error,
        file: String,
    },
    JSONEncodingError {
        error: serde_json::Error,
        file: String,
    },
}

pub fn load_json<T: for<'a> Deserialize<'a>>(path: &str, default: T) -> Result<T, StorageError> {
    match fs::read_to_string(path) {
        Ok(json_string) => serde_json::from_str(json_string.as_str()).map_err(|error| {
            StorageError::JSONDecodingError {
                error,
                file: path.to_string(),
            }
        }),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(default),
        Err(error) => Err(StorageError::FileReadError {
            error,
            file: path.to_string(),
        }),
    }
}

pub fn save_json<T: Serialize>(path: &str, value: &T) -> Result<(), StorageError> {
    let json =
        serde_json::to_string_pretty(value).map_err(|error| StorageError::JSONEncodingError {
            error,
            file: path.to_string(),
        })?;
    fs::write(path, json).map_err(|error| StorageError::FileWriteError {
        error,
        file: path.to_string(),
    })?;
    Ok(())
}
