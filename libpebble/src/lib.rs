mod config;
mod error;

pub use config::Config;
pub use error::Error;
use libc;
use nix::unistd;
use snafu::ResultExt;
use std::{fs, io};

pub fn setup() -> Result<(), Error> {
    if let Err(source) = fs::create_dir("/run/pebble") {
        if source.kind() != io::ErrorKind::AlreadyExists {
            return Err(Error::CreateDirectory {
                source: source,
                path: String::from("/run/pebble"),
            });
        }
    }

    let uid = unistd::Uid::from_raw(5000);
    let gid = unistd::Gid::from_raw(5000);

    unistd::chown("/run/pebble", Some(uid), Some(gid)).context(error::ChangeOwner {
        path: String::from("/run/pebble"),
    })?;

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
