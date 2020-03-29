use super::config::Config;
use super::error::{CreateDirectory, Error};
use snafu::ResultExt;
use std::path::{Path, PathBuf};
use std::{fs, io};

pub struct Create {
    root_dir: PathBuf,
}

impl Create {
    pub fn new(root_dir: &Path) -> Self {
        Create {
            root_dir: root_dir.to_owned(),
        }
    }

    pub fn create(&self, id: &str, _: Config) -> Result<(), Error> {
        let path = self.root_dir.join(id);

        match fs::create_dir(&path) {
            Err(err) if err.kind() == io::ErrorKind::AlreadyExists => Err(Error::AlreadyExists {
                id: String::from(id),
            }),
            result @ _ => result.context(CreateDirectory {
                path: path.to_string_lossy().to_owned().to_string(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Config;
    use super::Create;
    use super::Error;
    use tempfile::TempDir;

    #[test]
    fn create() {
        let root = TempDir::new().unwrap();

        Create::new(root.path())
            .create("foo", Config::default())
            .unwrap();

        assert!(root.path().exists());
    }

    #[test]
    fn create_already_exists() {
        let root = TempDir::new().unwrap();

        Create::new(root.path())
            .create("foo", Config::default())
            .unwrap();

        match Create::new(root.path()).create("foo", Config::default()) {
            Err(Error::AlreadyExists { .. }) => assert!(true),
            res @ _ => assert!(false, "expected Error::AlreadyExists, got {:?}", res),
        }
    }
}
