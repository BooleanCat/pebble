use libc;
use serde::Deserialize;
use structopt::clap::arg_enum;

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
