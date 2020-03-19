use libc;
use nix::unistd;
use serde::Deserialize;
use snafu::{ResultExt, Snafu};
use std::{fs, io};

#[serde(rename_all = "camelCase")]
#[derive(Deserialize, Debug)]
pub struct ConfigRoot {
    pub path: String,
    pub read_only: Option<bool>,
}

#[serde(rename_all = "camelCase")]
#[derive(Deserialize, Debug)]
pub struct Config {
    pub oci_version: String,
    pub root: ConfigRoot,
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu_display("not implemented")]
    NotImplemented,

    #[snafu_display("no such container")]
    NoSuchContainer,

    #[snafu_display("create directory: {}", "source")]
    CreateDirectory { source: io::Error },

    #[snafu_display("change owner: {}", "source")]
    ChangeOwner { source: nix::Error },
}

pub fn setup() -> Result<(), Error> {
    if let Err(err) = fs::create_dir("/run/pebble") {
        if err.kind() != io::ErrorKind::AlreadyExists {
            return Err(Error::CreateDirectory { source: err });
        }
    }

    let uid = unistd::Uid::from_raw(5000);
    let gid = unistd::Gid::from_raw(5000);

    unistd::chown("/run/pebble", Some(uid), Some(gid)).context(ChangeOwner {})?;

    Ok(())
}

pub fn state(_: &str) -> Result<(), Error> {
    Err(Error::NoSuchContainer)
}

pub fn create(_: &str, _: Config) -> Result<(), Error> {
    Err(Error::NotImplemented)
}

pub fn start(_: &str) -> Result<(), Error> {
    Err(Error::NoSuchContainer)
}

pub fn kill(_: &str, _: libc::c_int) -> Result<(), Error> {
    Err(Error::NoSuchContainer)
}

pub fn delete(_: &str) -> Result<(), Error> {
    Err(Error::NoSuchContainer)
}
