mod cli;

use std::path::Path;
use std::{env, process};

use cli::Config;

fn main() {
    println!("We're generating some digital art");

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    if let Err(e) = cli::run(Path::new(&config.dir)) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
    // TODO: Generate metadata (JSON) associated with each asset
    // TODO: Error handling if asset folders don't follow required convention
    // TODO: Update piece of metadata without regenerating assets
    // TODO: Wipe assets and regenerate everything
    // TODO: Add more assets to existing collection
}
