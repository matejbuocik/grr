use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead};
use log::{info, debug};

/// Search for a pattern in a file and display lines that contain it
#[derive(Parser)]
struct Cli {
    /// Pattern to look for
    pattern: String,

    /// Path to the file
    path: std::path::PathBuf,

    /// Case-insensitive search
    #[arg(short, long, default_value_t = false)]
    ignore_case: bool,

    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}

fn main() {
    let args = Cli::parse();
    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();

    if let Err(err) = run(&args) {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}

fn run(args: &Cli) -> io::Result<()> {
    let f = File::open(&args.path)?;
    let mut reader = io::BufReader::new(f);
    let mut line = String::new();
    let mut count = 1;

    let pattern = if args.ignore_case {args.pattern.to_lowercase()} else {args.pattern.clone()};

    info!("Starting!");
    while reader.read_line(&mut line)? > 0 {
        debug!("{}", line.trim_end());

        // to_lowercase allocates new string
        if line.contains(&pattern) || (args.ignore_case && line.to_lowercase().contains(&pattern)) {
            print!("{}: {}", count, line);
        }

        line.clear();
        count += 1;
    }

    info!("Finished.");
    Ok(())
}
