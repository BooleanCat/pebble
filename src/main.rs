use structopt::StructOpt;
use structopt::clap;

#[derive(Debug, StructOpt)]
#[structopt(name = "pebble", about = "An OCI container runtime")]
struct Opt {
    #[structopt(subcommand)]
    command: Subcommand,
}

#[derive(StructOpt, Debug)]
enum Subcommand {
    #[structopt(no_version, about = "Query the state of a container")]
    State {
        #[structopt(name = "container-id")]
        id: String,
    },

    #[structopt(no_version, about = "Kill a running container")]
    Kill {
        #[structopt(name = "container-id")]
        id: String,

        #[structopt(default_value = "SIGTERM")]
        signal: String,
    },
}

fn main() {
    match Opt::from_args().command {
        Subcommand::State{id} => { state(&id) },
        Subcommand::Kill{id, signal} => { kill(&id, &signal) },
    }
}

fn state(_: &str) {
    clap::Error::with_description("no such container", clap::ErrorKind::InvalidValue).exit();
}

fn kill(_: &str, _: &str) {
    clap::Error::with_description("no such container", clap::ErrorKind::InvalidValue).exit();
}
