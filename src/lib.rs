use std::{error, fmt, str};
use libc;


#[derive(Debug, Clone)]
pub struct ParseSignalError;

impl fmt::Display for ParseSignalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unknown signal")
    }
}

impl error::Error for ParseSignalError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Signal {
    SIGHUP = libc::SIGHUP as isize,
    SIGINT = libc::SIGINT as isize,
    SIGQUIT = libc::SIGQUIT as isize,
    SIGILL = libc::SIGILL as isize,
    SIGTRAP = libc::SIGTRAP as isize,
    SIGABRT = libc::SIGABRT as isize,  // also SIGIOT
    SIGBUS = libc::SIGBUS as isize,
    SIGFPE = libc::SIGFPE as isize,
    SIGKILL = libc::SIGKILL as isize,
    SIGUSR1 = libc::SIGUSR1 as isize,
    SIGSEGV = libc::SIGSEGV as isize,
    SIGUSR2 = libc::SIGUSR2 as isize,
    SIGPIPE = libc::SIGPIPE as isize,
    SIGALRM = libc::SIGALRM as isize,
    SIGTERM = libc::SIGTERM as isize,
    SIGSTKFLT = libc::SIGSTKFLT as isize,
    SIGCHLD = libc::SIGCHLD as isize,
    SIGCONT = libc::SIGCONT as isize,
    SIGSTOP = libc::SIGSTOP as isize,
    SIGTSTP = libc::SIGTSTP as isize,
    SIGTTIN = libc::SIGTTIN as isize,
    SIGTTOU = libc::SIGTTOU as isize,
    SIGURG = libc::SIGURG as isize,
    SIGXCPU = libc::SIGXCPU as isize,
    SIGXFSZ = libc::SIGXFSZ as isize,
    SIGVTALRM = libc::SIGVTALRM as isize,
    SIGPROF = libc::SIGPROF as isize,
    SIGWINCH = libc::SIGWINCH as isize,
    SIGIO = libc::SIGIO as isize, // also SIGPOLL
    SIGPWR = libc::SIGPWR as isize,
    SIGSYS = libc::SIGSYS as isize,
}

impl str::FromStr for Signal {
    type Err = ParseSignalError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "SIGHUP" => Ok(Signal::SIGHUP),
            "SIGINT" => Ok(Signal::SIGINT),
            "SIGQUIT" => Ok(Signal::SIGQUIT),
            "SIGILL" => Ok(Signal::SIGILL),
            "SIGTRAP" => Ok(Signal::SIGTRAP),
            "SIGABRT" | "SIGIOT" => Ok(Signal::SIGABRT),
            "SIGBUS" => Ok(Signal::SIGBUS),
            "SIGFPE" => Ok(Signal::SIGFPE),
            "SIGKILL" => Ok(Signal::SIGKILL),
            "SIGUSR1" => Ok(Signal::SIGUSR1),
            "SIGSEGV" => Ok(Signal::SIGSEGV),
            "SIGUSR2" => Ok(Signal::SIGUSR2),
            "SIGPIPE" => Ok(Signal::SIGPIPE),
            "SIGALRM" => Ok(Signal::SIGALRM),
            "SIGTERM" => Ok(Signal::SIGTERM),
            "SIGSTKFLT" => Ok(Signal::SIGSTKFLT),
            "SIGCHLD" => Ok(Signal::SIGCHLD),
            "SIGCONT" => Ok(Signal::SIGCONT),
            "SIGSTOP" => Ok(Signal::SIGSTOP),
            "SIGTSTP" => Ok(Signal::SIGTSTP),
            "SIGTTIN" => Ok(Signal::SIGTTIN),
            "SIGTTOU" => Ok(Signal::SIGTTOU),
            "SIGURG" => Ok(Signal::SIGURG),
            "SIGXCPU" => Ok(Signal::SIGXCPU),
            "SIGXFSZ" => Ok(Signal::SIGXFSZ),
            "SIGVTALRM" => Ok(Signal::SIGVTALRM),
            "SIGPROF" => Ok(Signal::SIGPROF),
            "SIGWINCH" => Ok(Signal::SIGWINCH),
            "SIGIO" | "SIGPOLL" => Ok(Signal::SIGIO),
            "SIGPWR" => Ok(Signal::SIGPWR),
            "SIGSYS" => Ok(Signal::SIGSYS),
            _ => Err(Self::Err{}),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Signal;

    #[test]
    fn from_str_sigabrt() {
        assert_eq!(Signal::SIGABRT, "SIGABRT".parse().unwrap());
        assert_eq!(Signal::SIGABRT, "SIGIOT".parse().unwrap());
    }

    #[test]
    fn from_str_sigio() {
        assert_eq!(Signal::SIGIO, "SIGIO".parse().unwrap());
        assert_eq!(Signal::SIGIO, "SIGPOLL".parse().unwrap());
    }
}
