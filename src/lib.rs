use std::fs::File;
use std::io::{self, BufRead};
use log::{info, debug};

pub fn search(path: std::path::PathBuf, pattern: String, ignore_case: bool) -> io::Result<()> {
    let f = File::open(&path)?;
    let mut reader = io::BufReader::new(f);
    let mut line = String::new();
    let mut count = 1;

    let pattern = if ignore_case {pattern.to_lowercase()} else {pattern};

    info!("Starting!");
    while reader.read_line(&mut line)? > 0 {
        debug!("{}", line.trim_end());

        // to_lowercase allocates new string
        if line.contains(&pattern) || (ignore_case && line.to_lowercase().contains(&pattern)) {
            print!("{}: {}", count, line);
        }

        line.clear();
        count += 1;
    }

    info!("Finished.");
    Ok(())
}
