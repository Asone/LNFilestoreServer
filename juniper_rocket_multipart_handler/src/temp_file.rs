use std::{path::PathBuf, fs::File, io::Write};

use multer::bytes::Bytes;

#[derive(Debug, PartialEq)]
pub struct TempFile{
    pub local_path: PathBuf,
    pub name: String,
    pub size: Option<usize>,
    pub content: Bytes
}

impl TempFile{
    pub fn get_local_path(&self) -> &PathBuf {
        &self.local_path
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_size(&self) -> &Option<usize> {
        &self.size
    }

    pub fn get_content(&self) -> &Bytes {
        &self.content
    }

    pub fn persist_file(&self) -> &PathBuf {
        let full_path = format!("{}/{}",&self.local_path.to_str().unwrap(),&self.name);
        let mut file = File::create(full_path).unwrap();
        file.write_all(&self.content);
        &self.local_path
    }
}
