use std::fs::File;
pub enum Stream {
    FileStream(FileStream),
    StdinStream(StdinStream),
}

pub struct FileStream {
    pub file: File,
}

pub struct StdinStream {}

impl FileStream {
    pub fn new(file_path: &str) -> Result<Self, std::io::Error> {
        let file = File::open(file_path)?;
        Ok(FileStream { file })
    }
}

impl StdinStream {
    pub fn new() -> Self {
        StdinStream {}
    }
}

impl Stream {
    pub fn new(file_path: Option<String>) -> Result<Stream, &'static str> {
        match file_path {
            Some(path) if path != "-" => {
                FileStream::new(&path)
                    .map(Stream::FileStream)
                    .map_err(|_| "Error opening file")
            },
            _ => {
                Ok(Stream::StdinStream(StdinStream::new()))
            }
        }
    }
}
