use clap::Parser;

/// Search for a pattern in a file and display lines that contain it
#[derive(Parser)]
pub struct Cli {
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

    if let Err(err) = grr::search(args.path, args.pattern, args.ignore_case) {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}
