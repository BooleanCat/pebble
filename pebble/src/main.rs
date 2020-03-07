use libpebble::Signal;
use std::fs::File;
use std::path::PathBuf;
use structopt::clap;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "pebble")]
#[structopt(setting = structopt::clap::AppSettings::ColoredHelp)]
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
        id: String,
    },
}

fn main() {
    match Opt::from_args().command {
        Subcommand::State { id } => state(&id),
        Subcommand::Create { id, path } => create(&id, &path),
        Subcommand::Start { id } => start(&id),
        Subcommand::Kill { id, signal } => kill(&id, signal),
        Subcommand::Delete { id } => delete(&id),
    }
}

fn state(id: &str) {
    match libpebble::state(id) {
        Err(err) => {
            clap::Error::with_description(
                &format!(r#"state "{}": {}"#, id, err),
                clap::ErrorKind::Io,
            )
            .exit();
        }
        _ => unreachable!(),
    }
}

fn create(id: &str, path: &PathBuf) {
    let file = File::open(path).unwrap_or_else(|err| {
        clap::Error::with_description(&format!("open {:?}: {}", path, err), clap::ErrorKind::Io)
            .exit()
    });

    let config = serde_json::from_reader(file).unwrap_or_else(|err| {
        clap::Error::with_description(
            &format!("parse {:?}: {}", path, err),
            clap::ErrorKind::Format,
        )
        .exit()
    });

    match libpebble::create(id, config) {
        Err(err) => {
            clap::Error::with_description(&format!("create: {}", err), clap::ErrorKind::Io).exit();
        }
        _ => unreachable!(),
    }
}

fn start(id: &str) {
    match libpebble::start(id) {
        Err(err) => {
            clap::Error::with_description(
                &format!(r#"start "{}": {}"#, id, err),
                clap::ErrorKind::Io,
            )
            .exit();
        }
        _ => unreachable!(),
    }
}

fn kill(id: &str, signal: Signal) {
    match libpebble::kill(id, signal) {
        Err(err) => {
            clap::Error::with_description(
                &format!(r#"kill "{}": {}"#, id, err),
                clap::ErrorKind::Io,
            )
            .exit();
        }
        _ => unreachable!(),
    }
}

fn delete(id: &str) {
    match libpebble::delete(id) {
        Err(err) => {
            clap::Error::with_description(
                &format!(r#"delete "{}": {}"#, id, err),
                clap::ErrorKind::Io,
            )
            .exit();
        }
        _ => unreachable!(),
    }
}
