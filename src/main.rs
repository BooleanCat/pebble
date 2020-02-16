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
}

fn main() {
    match Opt::from_args().command {
        Subcommand::State{id} => { state(&id) },
    }
}

fn state(_: &str) {
    clap::Error::with_description("no such container", clap::ErrorKind::InvalidValue).exit();
}
