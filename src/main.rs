use std::env;
use std::ffi::CString;

use nix::fcntl::{fcntl, F_GETFD};
use nix::sys::stat::{fchmod, mode_t, Mode};
use nix::unistd::execvp;

const PROGNAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn usage() -> ! {
    eprintln!(
        r#"{} {}
usage: <MODE> <FD> <...> -- <COMMAND> <...>"#,
        PROGNAME, VERSION,
    );
    std::process::exit(1);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() < 4 {
        usage()
    }

    let bits: mode_t = u32::from_str_radix(&args[0], 8).unwrap();
    let mode = Mode::from_bits(bits).unwrap();

    let sep = args[1..]
        .iter()
        .position(|arg| arg == "--")
        .expect("separator (--) not found");

    let fd: Vec<i32> = args[1..=sep]
        .iter()
        .map(|arg| arg.parse().unwrap())
        .filter(|arg| fcntl(*arg, F_GETFD).is_ok())
        .collect();

    let argv: Vec<_> = args[(sep + 2)..]
        .iter()
        .map(|arg| CString::new(arg.as_str()).unwrap())
        .collect();

    for x in &fd {
        fchmod(*x, mode)?;
    }

    execvp(&argv[0], &argv)?;

    unreachable!()
}
