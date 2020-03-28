use libc;
use libpebble;
use std::fs::File;
use std::path::PathBuf;
use structopt::StructOpt;
use structopt::{clap, clap::arg_enum};

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
    /// Perform one time system setup (requires root)
    Setup {
        #[structopt(long = "uid", default_value = "0x90000")]
        uid: libc::uid_t,

        #[structopt(long = "gid", default_value = "0x90000")]
        gid: libc::gid_t,
    },

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
        Subcommand::Setup { uid, gid } => setup(uid, gid),
        Subcommand::State { id } => state(&id),
        Subcommand::Create { id, path } => create(&id, &path),
        Subcommand::Start { id } => start(&id),
        Subcommand::Kill { id, signal } => kill(&id, signal),
        Subcommand::Delete { id } => delete(&id),
    }
}

fn setup(uid: libc::uid_t, gid: libc::gid_t) {
    if let Err(err) = libpebble::setup(uid, gid) {
        clap::Error::with_description(&format!("setup: {}", err), clap::ErrorKind::Io).exit();
    }
}

fn state(id: &str) {
    if let Err(err) = libpebble::state(id) {
        clap::Error::with_description(&format!(r#"state "{}": {}"#, id, err), clap::ErrorKind::Io)
            .exit();
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

    if let Err(err) = libpebble::create(id, config) {
        clap::Error::with_description(&format!("create: {}", err), clap::ErrorKind::Io).exit();
    }
}

fn start(id: &str) {
    if let Err(err) = libpebble::start(id) {
        clap::Error::with_description(&format!(r#"start "{}": {}"#, id, err), clap::ErrorKind::Io)
            .exit();
    }
}

fn kill(id: &str, signal: Signal) {
    if let Err(err) = libpebble::kill(id, signal.into()) {
        clap::Error::with_description(&format!(r#"kill "{}": {}"#, id, err), clap::ErrorKind::Io)
            .exit();
    }
}

fn delete(id: &str) {
    if let Err(err) = libpebble::delete(id) {
        clap::Error::with_description(&format!(r#"delete "{}": {}"#, id, err), clap::ErrorKind::Io)
            .exit();
    }
}

arg_enum! {
    #[derive(Debug)]
    pub enum Signal {
        SIGHUP,
        SIGINT,
        SIGQUIT,
        SIGILL,
        SIGTRAP,
        SIGABRT,
        SIGIOT,
        SIGBUS,
        SIGFPE,
        SIGKILL,
        SIGUSR1,
        SIGSEGV,
        SIGUSR2,
        SIGPIPE,
        SIGALRM,
        SIGTERM,
        SIGSTKFLT,
        SIGCHLD,
        SIGCONT,
        SIGSTOP,
        SIGTSTP,
        SIGTTIN,
        SIGTTOU,
        SIGURG,
        SIGXCPU,
        SIGXFSZ,
        SIGVTALRM,
        SIGPROF,
        SIGWINCH,
        SIGIO,
        SIGPOLL,
        SIGPWR,
        SIGSYS,
    }
}

impl Into<libc::c_int> for Signal {
    fn into(self) -> libc::c_int {
        match self {
            Self::SIGHUP => libc::SIGHUP,
            Self::SIGINT => libc::SIGINT,
            Self::SIGQUIT => libc::SIGQUIT,
            Self::SIGILL => libc::SIGILL,
            Self::SIGTRAP => libc::SIGTRAP,
            Self::SIGABRT => libc::SIGABRT,
            Self::SIGIOT => libc::SIGIOT,
            Self::SIGBUS => libc::SIGBUS,
            Self::SIGFPE => libc::SIGFPE,
            Self::SIGKILL => libc::SIGKILL,
            Self::SIGUSR1 => libc::SIGUSR1,
            Self::SIGSEGV => libc::SIGSEGV,
            Self::SIGUSR2 => libc::SIGUSR2,
            Self::SIGPIPE => libc::SIGPIPE,
            Self::SIGALRM => libc::SIGALRM,
            Self::SIGTERM => libc::SIGTERM,
            Self::SIGSTKFLT => libc::SIGSTKFLT,
            Self::SIGCHLD => libc::SIGCHLD,
            Self::SIGCONT => libc::SIGCONT,
            Self::SIGSTOP => libc::SIGSTOP,
            Self::SIGTSTP => libc::SIGTSTP,
            Self::SIGTTIN => libc::SIGTTIN,
            Self::SIGTTOU => libc::SIGTTOU,
            Self::SIGURG => libc::SIGURG,
            Self::SIGXCPU => libc::SIGXCPU,
            Self::SIGXFSZ => libc::SIGXFSZ,
            Self::SIGVTALRM => libc::SIGVTALRM,
            Self::SIGPROF => libc::SIGPROF,
            Self::SIGWINCH => libc::SIGWINCH,
            Self::SIGIO => libc::SIGIO,
            Self::SIGPOLL => libc::SIGPOLL,
            Self::SIGPWR => libc::SIGPWR,
            Self::SIGSYS => libc::SIGSYS,
        }
    }
}
