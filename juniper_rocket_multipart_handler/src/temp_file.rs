use multer::bytes::Bytes;
use std::fs;
use std::io::Error;
use std::{fs::File, io::Write, path::PathBuf};

/// A buffered file from file upload.
///
/// Please, **do note** that this struct
/// is different from original [`TempFile`] provided
/// by original's rocket implementation as it does not
/// intend to rely on a specific lifetime specification
/// other than the graphQL request processing lifetime.
///
/// Current [`TempFile`] struct provides with a `persist_file`
/// method that will write the file on the filesystem based on the
/// `local_path` of the struct.
///
/// default `local_path` provided is based on [`env::temp_dir()`](https://doc.rust-lang.org/nightly/std/env/fn.temp_dir.html) value
#[derive(Debug, PartialEq, Clone)]
pub struct TempFile {
    pub local_path: PathBuf,
    pub name: String,
    pub size: Option<usize>,
    pub content: Bytes,
}

impl TempFile {
    /// Gets the local path where file should be persisted
    pub fn get_local_path(&self) -> &PathBuf {
        &self.local_path
    }

    /// Sets the local path to be used for persisting
    pub fn set_local_path(&mut self, path: PathBuf) -> &PathBuf {
        self.local_path = path;
        &self.local_path
    }

    /// Gets the file name
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Gets the file  size
    pub fn get_size(&self) -> &Option<usize> {
        &self.size
    }

    /// Gets the file content
    pub fn get_content(&self) -> &Bytes {
        &self.content
    }

    fn path_checker(path: &str) -> Result<(), Error> {
        fs::create_dir_all(path)
    }

    /// Persists the file to the local_path property
    pub fn persist_file(&self) -> Result<&PathBuf, Error> {
        let full_path = format!("{}/{}", &self.local_path.to_str().unwrap(), &self.name);
        let file = File::create(full_path);

        match file {
            Ok(mut file) => {
                let result = file.write_all(&self.content);
                match result {
                    Ok(_) => Ok(&self.local_path),
                    Err(error) => Err(error),
                }
            }
            Err(error) => Err(error),
        }
    }

    /// persists file to a given location
    pub fn persist_to(&self, path: &str) -> Result<PathBuf, Error> {
        match Self::path_checker(path) {
            Ok(_) => {
                let full_path = format!("{}/{}", path, &self.name);
                let path_buf = PathBuf::from(&full_path);
                let file = File::create(&full_path);

                match file {
                    Ok(mut file) => {
                        let result = file.write_all(&self.content);
                        match result {
                            Ok(_) => Ok(path_buf),
                            Err(error) => Err(error),
                        }
                    }
                    Err(error) => Err(error),
                }
            }
            Err(error) => Err(error),
        }
    }
}
