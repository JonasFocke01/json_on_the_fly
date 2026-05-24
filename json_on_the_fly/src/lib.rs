use anyhow::Result;
pub use json_on_the_fly_derive::JsonOnTheFly;
use serde::{de::DeserializeOwned, Serialize};
use std::{fmt::Display, fs::OpenOptions, io::Write, path::PathBuf};

#[derive(Debug)]
pub enum JsonStoreError {
    FileNotFound,
    PathNotValid,
    FilecontentNotValid,
    #[allow(non_camel_case_types)]
    FilecontentNotValid_CreatedBackupfile,
    #[allow(non_camel_case_types)]
    FilecontentNotValid_CouldNotCreateBackupfile,
}

impl Display for JsonStoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for JsonStoreError {
    fn description(&self) -> &str {
        "Json Store Error"
    }
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
    fn cause(&self) -> Option<&dyn std::error::Error> {
        None
    }
}

pub trait JsonOnTheFly: Serialize + DeserializeOwned + Default + Clone {
    /// Where this trait looks for the associated file
    fn db_file_path() -> PathBuf;

    /// Reads the struct file and returns the struct
    fn load() -> Result<Self, JsonStoreError> {
        match Self::read_and_deserialize_file() {
            Err(e) => Err(e),
            Ok(s) => Ok(s),
        }
    }
    /// Writes the `default` for the struct to the file
    fn setup() -> Result<Self> {
        let t = Self::default();
        t.serialize_and_write_file()?;

        Ok(t)
    }

    // TODO: migrate funtion

    /// Serializes and writes the struct to the associated file
    fn write(&self) -> Result<bool> {
        self.serialize_and_write_file()?;
        Ok(true)
    }

    /// Copies the file associated to the struct and saves it as "<name>.backup.json"
    fn backup_db_file() -> Result<u64> {
        let mut backup_file_path = Self::db_file_path().clone();
        backup_file_path.set_extension("backup.json");
        Ok(std::fs::copy(Self::db_file_path().clone(), backup_file_path).unwrap())
    }

    /// Internal function to serialize and write the struct
    /// Use the `.write()` command instead
    fn serialize_and_write_file(&self) -> Result<()> {
        let serialized_data = serde_json::to_string_pretty(&self)?;
        let Ok(path_as_string) = Self::db_file_path().clone().into_os_string().into_string() else {
            return Err(JsonStoreError::PathNotValid.into());
        };

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path_as_string)
            .unwrap();

        write!(file, "{serialized_data}")?;

        Ok(())
    }

    /// Internal function to serialize and write the struct
    /// Use the `.load()` command instead
    fn read_and_deserialize_file() -> Result<Self, JsonStoreError> {
        let Ok(parsed_db_file_path) = Self::db_file_path().clone().into_os_string().into_string()
        else {
            return Err(JsonStoreError::PathNotValid);
        };
        let Ok(data_str) = std::fs::read_to_string(parsed_db_file_path.as_str()) else {
            return Err(JsonStoreError::FileNotFound);
        };
        let Ok(data) = serde_json::from_str(&data_str) else {
            return Err(JsonStoreError::FilecontentNotValid);
        };
        Ok(data)
    }
}

/// Small helper function to return the home-dir-path for use in `db_file_path`
pub fn home_dir() -> Result<PathBuf> {
    let home = std::env::var("HOME")?;
    Ok(PathBuf::from(home))
}
