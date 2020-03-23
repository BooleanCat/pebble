use libc::{c_int, gid_t, uid_t};
use nix::unistd;
use serde::{Deserialize, Serialize};
use snafu::{ResultExt, Snafu};
use std::{fs, io, path::Path};

const DEFAULT_RUN_DIR: &'static str = "/run/pebble";

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigRoot {
    pub path: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub oci_version: String,
    pub root: ConfigRoot,
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu_display("no such container")]
    NotFound,

    #[snafu_display("container id already in use")]
    AlreadyExists,

    #[snafu_display("create directory: {}", "source")]
    CreateDirectory { source: io::Error },

    #[snafu_display("remove directory: {}", "source")]
    RemoveDirectory { source: io::Error },

    #[snafu_display("list directory: {}", "source")]
    ListDirectory { source: io::Error },

    #[snafu_display("create file: {}", "source")]
    CreateFile { source: io::Error },

    #[snafu_display("write JSON")]
    WriteJSON { source: serde_json::error::Error },

    #[snafu_display("change owner: {}", "source")]
    ChangeOwner { source: nix::Error },
}

pub fn setup(uid: uid_t, gid: gid_t) -> Result<(), Error> {
    if let Err(err) = fs::create_dir(DEFAULT_RUN_DIR) {
        if err.kind() != io::ErrorKind::AlreadyExists {
            return Err(Error::CreateDirectory { source: err });
        }
    }

    let uid = unistd::Uid::from_raw(uid);
    let gid = unistd::Gid::from_raw(gid);

    unistd::chown(DEFAULT_RUN_DIR, Some(uid), Some(gid)).context(ChangeOwner {})?;

    Ok(())
}

pub fn state(_: &str) -> Result<(), Error> {
    Err(Error::NotFound)
}

pub fn create(id: &str, config: Config) -> Result<(), Error> {
    if let Err(err) = fs::create_dir(Path::new(DEFAULT_RUN_DIR).join(id)) {
        if err.kind() == io::ErrorKind::AlreadyExists {
            return Err(Error::CreateDirectory { source: err });
        }
    }

    serde_json::to_writer(
        &fs::File::create(Path::new(DEFAULT_RUN_DIR).join(id).join("config.json"))
            .context(CreateFile {})?,
        &config,
    )
    .context(WriteJSON {})?;

    Ok(())
}

pub fn start(_: &str) -> Result<(), Error> {
    Err(Error::NotFound)
}

pub fn kill(_: &str, _: c_int) -> Result<(), Error> {
    Err(Error::NotFound)
}

pub fn delete(id: &str) -> Result<(), Error> {
    match fs::remove_dir_all(Path::new(DEFAULT_RUN_DIR).join(id)) {
        Err(err) if err.kind() == io::ErrorKind::NotFound => Err(Error::NotFound),
        Err(err) => Err(Error::RemoveDirectory { source: err }),
        Ok(_) => Ok(()),
    }
}

pub fn list() -> Result<(), Error> {
    let dirs = fs::read_dir(DEFAULT_RUN_DIR)
        .context(ListDirectory {})?
        .collect::<Result<Vec<_>, _>>()
        .context(ListDirectory {})?;

    for entry in dirs {
        if let Some(base) = entry.path().as_path().file_name() {
            println!("{}", base.to_string_lossy());
        }
    }

    Ok(())
}
