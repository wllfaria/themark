use std::path::PathBuf;

pub enum DocumentPath {
    File(PathBuf),
    Dir(PathBuf),
}

#[derive(Debug)]
pub enum DocumentPathError {
    NotFound(String),
}

impl std::error::Error for DocumentPathError {}

impl std::fmt::Display for DocumentPathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DocumentPathError::NotFound(msg) => f.write_str(msg),
        }
    }
}

impl TryFrom<String> for DocumentPath {
    type Error = DocumentPathError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let path = PathBuf::from(&value);
        if !path.exists() {
            return Err(DocumentPathError::NotFound(format!(
                "unable to find {value}"
            )));
        }

        let doc_path = match path.is_dir() {
            true => Self::Dir(path),
            false => Self::File(path),
        };

        Ok(doc_path)
    }
}
