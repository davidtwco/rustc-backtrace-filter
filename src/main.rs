use atty;
use clap::Parser;
use lazy_static;
use regex::RegexSet;
use std::{
    error,
    ffi::OsStr,
    fs::{File, OpenOptions},
    io::{self, BufRead, Write},
    path::PathBuf,
};

const BACKTRACE_START: &'static str = "stack backtrace:";
const BACKTRACE_END: &'static str = "";

const DRIVER_PATTERNS: &'static [&'static str] = &[r"rustc_driver_impl\[[\w\d]+\]"];
const FN_TRAIT_PATTERNS: &'static [&'static str] = &[r"core(?:\[[\w\d]+\])?::ops::function"];
const PANIC_PATTERNS: &'static [&'static str] = &[
    r"std(?:\[[\w\d]+\])?::backtrace_rs",
    r"std(?:\[[\w\d]+\])?::sys_common::backtrace",
    r"std(?:\[[\w\d]+\])?::panicking",
    r"std(?:\[[\w\d]+\])?::panic",
    r"core(?:\[[\w\d]+\])?::fmt",
    r"std(?:\[[\w\d]+\])?::io::Write::write_fmt",
    r"rustc_driver_impl\[[\w\d]+\]::install_ice_hook",
];
const QUERY_SYSTEM_PATTERNS: &'static [&'static str] = &[
    r"rustc_query_system\[[\w\d]+\]",
    r"rustc_query_impl\[[\w\d]+\]",
    r"rustc_middle\[[\w\d]+\]::query",
];
const THREAD_PATTERNS: &'static [&'static str] = &[
    "__pthread_joiner_wake",
    r"std(?:\[[\w\d]+\])?::sys::unix::thread",
];
const TIMING_PATTERNS: &'static [&'static str] = &[
    r"rustc_data_structures(?:\[[\w\d]+\])?::profiling",
    r"<rustc_session(?:\[[\w\d]+\])?::session::Session>::time",
];
const TLS_PATTERNS: &'static [&'static str] = &[r"rustc_middle(?:\[[\w\d]+\])?::ty::context::tls"];

lazy_static::lazy_static! {
    static ref DRIVER_REGEXES: RegexSet =
         RegexSet::new(DRIVER_PATTERNS).expect("compiling driver regexes failed");
    static ref FN_TRAIT_REGEXES: RegexSet =
         RegexSet::new(FN_TRAIT_PATTERNS).expect("compiling fn trait regexes failed");
    static ref PANIC_REGEXES: RegexSet =
         RegexSet::new(PANIC_PATTERNS).expect("compiling panic regexes failed");
    static ref QUERY_SYSTEM_REGEXES: RegexSet =
         RegexSet::new(QUERY_SYSTEM_PATTERNS).expect("compiling query system regexes failed");
    static ref THREAD_REGEXES: RegexSet =
         RegexSet::new(THREAD_PATTERNS).expect("compiling thread regexes failed");
    static ref TIMING_REGEXES: RegexSet =
         RegexSet::new(TIMING_PATTERNS).expect("compiling timing regexes failed");
    static ref TLS_REGEXES: RegexSet =
         RegexSet::new(TLS_PATTERNS).expect("compiling tls regexes failed");
}

/// Returns `true` if the file type is a fifo.
#[cfg(not(target_family = "unix"))]
fn is_fifo(_: std::fs::FileType) -> bool {
    false
}

/// Returns `true` if the file type is a fifo.
#[cfg(target_family = "unix")]
fn is_fifo(file_type: std::fs::FileType) -> bool {
    use std::os::unix::fs::FileTypeExt;
    file_type.is_fifo()
}

/// Wrapper around output writer which handles differences between stdout, file and pipe outputs.
pub(crate) enum Output {
    Stdout(io::Stdout),
    File(File),
    Pipe(File),
}

impl Output {
    /// Create a `Output` from the input path (or "-" for stdout).
    pub(crate) fn new(path: &OsStr) -> io::Result<Self> {
        if path == "-" {
            return Ok(Output::Stdout(io::stdout()));
        }

        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)?;
        if is_fifo(file.metadata()?.file_type()) {
            Ok(Output::File(file))
        } else {
            Ok(Output::Pipe(file))
        }
    }
}

impl Write for Output {
    fn flush(&mut self) -> io::Result<()> {
        match self {
            Output::Stdout(stdout) => stdout.flush(),
            Output::Pipe(pipe) => pipe.flush(),
            Output::File(file) => file.flush(),
        }
    }

    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self {
            Output::Stdout(stdout) => stdout.write(buf),
            Output::Pipe(pipe) => pipe.write(buf),
            Output::File(file) => file.write(buf),
        }
    }
}

#[derive(Parser)]
struct Args {
    /// Path where filtered input should be output, defaults to stdout.
    #[arg(short, value_hint = clap::ValueHint::FilePath)]
    output: Option<PathBuf>,
    /// Should driver-related lines be filtered from backtrace?
    #[arg(short, long, default_value_t = true)]
    driver: bool,
    /// Should function-trait-related lines be filtered from backtrace?
    #[arg(short, long, default_value_t = true)]
    fn_trait: bool,
    /// Should panic/backtrace-related lines be filtered from backtrace?
    #[arg(short, long, default_value_t = true)]
    panics: bool,
    /// Should query-system-related lines be filtered from backtrace?
    #[arg(short, long, default_value_t = true)]
    query_system: bool,
    /// Should thread-related lines be filtered from backtrace?
    #[arg(short, long, default_value_t = true)]
    threads: bool,
    /// Should timing-related lines be filtered from backtrace?
    #[arg(short, long, default_value_t = true)]
    timing: bool,
    /// Should tls-related lines be filtered from backtrace?
    #[arg(short, long, default_value_t = true)]
    tls: bool,
}

fn main() -> Result<(), Box<dyn error::Error>> {
    if atty::is(atty::Stream::Stdin) {
        return Ok(());
    }

    let args = Args::parse();
    let mut output = args
        .output
        .map(|path| Output::new(path.as_os_str()))
        .unwrap_or_else(|| Output::new(OsStr::new("-")))?;

    let mut found_backtrace = false;
    let mut skip_until_next_header = false;
    for line in io::stdin().lock().lines() {
        let line = line?;

        if line == BACKTRACE_START {
            found_backtrace = true;

            // We always want to print the start of the backtrace, there's no reason to skip in
            // this line.
            writeln!(output, "{line}")?;
            continue;
        } else if line == BACKTRACE_END {
            found_backtrace = false;
        }

        if !found_backtrace {
            // Print every line that isn't part of a backtrace.
            writeln!(output, "{line}")?;
            continue;
        }

        // Backtraces look like this, but are sometimes only one line:
        //
        // ```
        //   <NUM>:  <ADDR>: - <PATH>
        //     at <PATH>:<LINE>:<COL>
        // ```
        //
        // We'll skip lines based on the contents of the header.
        let is_header = line.find("0x").is_some();
        if is_header {
            skip_until_next_header = false;

            if args.driver {
                skip_until_next_header |= DRIVER_REGEXES.is_match(&line);
            }

            if !skip_until_next_header && args.fn_trait {
                skip_until_next_header |= FN_TRAIT_REGEXES.is_match(&line);
            }

            if !skip_until_next_header && args.panics {
                skip_until_next_header |= PANIC_REGEXES.is_match(&line);
            }

            if !skip_until_next_header && args.query_system {
                skip_until_next_header |= QUERY_SYSTEM_REGEXES.is_match(&line);
            }

            if !skip_until_next_header && args.threads {
                skip_until_next_header |= THREAD_REGEXES.is_match(&line);
            }

            if !skip_until_next_header && args.timing {
                skip_until_next_header |= TIMING_REGEXES.is_match(&line);
            }

            if !skip_until_next_header && args.tls {
                skip_until_next_header |= TLS_REGEXES.is_match(&line);
            }
        }

        if skip_until_next_header {
            continue;
        } else {
            writeln!(output, "{line}")?;
        }
    }

    Ok(())
}
