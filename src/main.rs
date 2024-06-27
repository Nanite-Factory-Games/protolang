use std::{fs, path::PathBuf};

use anyhow::Ok;
use clap::Parser;
use protolang::{parsing::program::protolang_file, writer::WRITERS};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The path to search for files to compile
    #[arg()]
    path: PathBuf,

    #[arg(long, default_value = "./build")]
    output: PathBuf
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let files = fs::read_dir(args.path)?;
    
    let mut parsed_files = Vec::new();
    for file in files {
        let file = file?;
        let parsed_file = protolang_file(fs::read_to_string(file.path())?.as_str())?;
        parsed_files.push(parsed_file);
    }

    for (language, writer) in WRITERS.entries() {
        println!("Writing {} files", language);
        let output_folder = args.output.join(language);
        fs::create_dir_all(&output_folder)?;
        writer(&output_folder, &parsed_files)?;
    }

    Ok(())
}
