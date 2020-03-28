mod config;
mod error;

pub use config::Config;
pub use error::Error;
use libc::{c_int, gid_t, uid_t};
use nix::unistd;
use snafu::ResultExt;
use std::{fs, io};

const RUN_DIR: &'static str = "/run/pebble";

pub fn setup(uid: uid_t, gid: gid_t) -> Result<(), Error> {
    if let Err(source) = fs::create_dir(RUN_DIR) {
        if source.kind() != io::ErrorKind::AlreadyExists {
            return Err(Error::CreateDirectory {
                source: source,
                path: String::from(RUN_DIR),
            });
        }
    }

    let uid = unistd::Uid::from_raw(uid);
    let gid = unistd::Gid::from_raw(gid);

    unistd::chown(RUN_DIR, Some(uid), Some(gid)).context(error::ChangeOwner {
        path: String::from(RUN_DIR),
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

pub fn kill(_: &str, _: c_int) -> Result<(), Error> {
    Err(Error::NoSuchContainer)
}

pub fn delete(_: &str) -> Result<(), Error> {
    Err(Error::NoSuchContainer)
}
