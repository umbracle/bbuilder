use clap::Parser;
use std::path::PathBuf;
use std::process;

#[derive(Parser, Debug)]
#[command(name = "fetcher")]
#[command(about = "A go-getter-like tool to fetch files from various sources")]
struct Args {
    /// Source URL to fetch from
    source: String,

    /// Destination path to save the file
    destination: PathBuf,
}

fn main() {
    let args = Args::parse();

    let mut progress = fetcher::ConsoleProgressTracker::new();

    if let Err(e) = fetcher::fetch_with_progress(&args.source, &args.destination, &mut progress) {
        eprintln!("Error: {}", e);

        // Print the error chain
        let mut source = e.source();
        while let Some(err) = source {
            eprintln!("  Caused by: {}", err);
            source = err.source();
        }

        process::exit(1);
    }

    println!("Successfully downloaded to: {}", args.destination.display());
}
