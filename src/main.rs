use std::{fs, path::PathBuf};

use anyhow::Ok;
use clap::Parser;
use protolang::parsing::program::protolang_file;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The path to search for files to compile
    #[arg()]
    path: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let files = fs::read_dir(args.path)?;
    
    for file in files {
        let file = file?;
        let parsed_file = protolang_file(fs::read_to_string(file.path())?.as_str())?;
        dbg!(parsed_file);
    }

    Ok(())
}
