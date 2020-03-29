use snafu::Snafu;
use std::io;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    #[snafu(display("not implemented"))]
    NotImplemented,

    #[snafu(display(r#"no such container "{}""#, id))]
    NotFound { id: String },

    #[snafu(display(r#"already exists "{}""#, id))]
    AlreadyExists { id: String },

    #[snafu(display(r#"create directory "{}": {}"#, path, source))]
    CreateDirectory { source: io::Error, path: String },

    #[snafu(display(r#"change owner "{}": {}"#, path, source))]
    ChangeOwner { source: nix::Error, path: String },

    #[snafu(display(r#"remove directory "{}": {}"#, path, source))]
    RemoveDirectory { source: io::Error, path: String },
}
