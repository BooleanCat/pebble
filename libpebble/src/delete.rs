use super::error::{Error, RemoveDirectory};
use snafu::ResultExt;
use std::path::{Path, PathBuf};
use std::{fs, io};

pub struct Delete {
    root_dir: PathBuf,
}

impl Delete {
    pub fn new(root_dir: &Path) -> Self {
        Delete {
            root_dir: root_dir.to_owned(),
        }
    }

    pub fn delete(&self, id: &str) -> Result<(), Error> {
        let path = self.root_dir.join(id);

        match fs::remove_dir_all(&path) {
            Err(err) if err.kind() == io::ErrorKind::NotFound => {
                Err(Error::NotFound { id: id.to_owned() })
            }
            result @ _ => result.context(RemoveDirectory {
                path: path.to_string_lossy().to_owned().to_string(),
            }),
        }
    }
}
