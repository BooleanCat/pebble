use std::path::PathBuf;
use std::fs::File;
use structopt::StructOpt;
use structopt::clap;
use libc;
use libpebble::{Signal, Config};

#[derive(Debug, StructOpt)]
#[structopt(name = "pebble")]
/// An OCI container runtime
struct Opt {
    #[structopt(subcommand)]
    command: Subcommand,
}

#[derive(StructOpt, Debug)]
enum Subcommand {
    #[structopt(no_version)]
    /// Query the state of a container
    State {
        #[structopt(name = "container-id")]
        id: String,
    },

    #[structopt(no_version)]
    /// Create a new container
    Create {
        #[structopt(name = "container-id")]
        id: String,

        #[structopt(name = "path-to-bundle", parse(from_os_str))]
        path: PathBuf,
    },

    #[structopt(no_version)]
    /// Start a created container
    Start {
        #[structopt(name = "container-id")]
        id: String,
    },

    #[structopt(no_version)]
    /// Kill a running container
    Kill {
        #[structopt(name = "container-id")]
        id: String,

        #[structopt(default_value = "SIGTERM")]
        signal: Signal,
    },

    #[structopt(no_version)]
    /// Delete a container
    Delete {
        #[structopt(name = "container-id")]
        id: String
    }
}

fn main() {
    match Opt::from_args().command {
        Subcommand::State{id} => state(&id),
        Subcommand::Create{id, path} => create(&id, &path),
        Subcommand::Start{id} => start(&id),
        Subcommand::Kill{id, signal} => kill(&id, signal.into()),
        Subcommand::Delete{id} => delete(&id),
    }
}

fn state(_: &str) {
    clap::Error::with_description("no such container", clap::ErrorKind::InvalidValue).exit();
}

fn create(_: &str, path: &PathBuf) {
    let file = File::open(path).unwrap_or_else(|err| clap::Error::with_description(
        &format!("open {:?}: {}", path, err),
        clap::ErrorKind::Io
    ).exit());

    let _: Config = serde_json::from_reader(file).unwrap_or_else(|err| clap::Error::with_description(
        &format!("parse {:?}: {}", path, err),
        clap::ErrorKind::Format
    ).exit());

    clap::Error::with_description("not implemented", clap::ErrorKind::InvalidValue).exit();
}

fn start(_: &str) {
    clap::Error::with_description("no such container", clap::ErrorKind::InvalidValue).exit();
}

fn kill(_: &str, _: libc::c_int) {
    clap::Error::with_description("no such container", clap::ErrorKind::InvalidValue).exit();
}

fn delete(_: &str) {
    clap::Error::with_description("no such container", clap::ErrorKind::InvalidValue).exit();
}
