use clap::Parser;
use std::fs::{self, OpenOptions};
use std::io::{Seek, SeekFrom, Write};

use ips::Patch;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct InOut {
    #[arg(short, long)]
    in_file: String,

    #[arg(short, long)]
    out_file: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = InOut::parse();

    let mut rom = OpenOptions::new()
        .read(true)
        .write(true)
        .open(args.in_file)?;
    let patch_contents = fs::read(args.out_file)?;
    let patch = Patch::parse(&patch_contents)?;

    for hunk in patch.hunks() {
        rom.seek(SeekFrom::Start(hunk.offset() as u64))?;
        rom.write_all(hunk.payload())?;
    }
    if let Some(truncation) = patch.truncation() {
        rom.set_len(truncation as u64)?;
    }

    Ok(())
}
