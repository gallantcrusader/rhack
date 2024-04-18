use clap::{arg, Command};
use std::fs::{self, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use xdelta3::decode;

use ips::Patch;

fn cli() -> Command {
    Command::new("rhack")
        .about("Patch/Rom hacking utility.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("ips")
                .about("Uses IPS patching")
                .args([
                    arg!(-i --"input-file" <FILE> "File to patch")
                        .value_parser(clap::value_parser!(PathBuf)),
                    arg!(-p --"patch-file" <FILE> "Patch file to use")
                        .value_parser(clap::value_parser!(PathBuf)),
                    arg!(-o --"output-file" <FILE> "Output file")
                        .value_parser(clap::value_parser!(PathBuf)),
                ])
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("xdelta")
                .about("Uses XDELTA patching")
                .args([
                    arg!(-i --"input-file" <FILE> "File to patch")
                        .value_parser(clap::value_parser!(PathBuf)),
                    arg!(-p --"patch-file" <FILE> "Patch file to use")
                        .value_parser(clap::value_parser!(PathBuf)),
                    arg!(-o --"output-file" <FILE> "Output file")
                        .value_parser(clap::value_parser!(PathBuf)),
                ])
                .arg_required_else_help(true),
        )
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("ips", sub_m)) => {
            let input = sub_m.get_one::<PathBuf>("input-file").unwrap();
            let patch = sub_m.get_one::<PathBuf>("patch-file").unwrap();
            let output = sub_m.get_one::<PathBuf>("output-file").unwrap();

            let mut rom = OpenOptions::new().read(true).write(true).open(input)?;
            let mut new_file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(output)?;
            let mut buf: Vec<u8> = vec![];
            rom.read_to_end(&mut buf)?;
            new_file.write_all(&mut buf)?;

            let ptc = fs::read(patch)?;
            let patch = Patch::parse(&ptc)?;

            for hunk in patch.hunks() {
                new_file.seek(SeekFrom::Start(hunk.offset() as u64))?;
                new_file.write_all(hunk.payload())?;
            }
            if let Some(truncation) = patch.truncation() {
                new_file.set_len(truncation as u64)?;
            }

            rom.sync_all()?;
            new_file.sync_all()?;
        }
        Some(("xdelta", sub_m)) => {
            let input = sub_m.get_one::<PathBuf>("input-file").unwrap();
            let patch = sub_m.get_one::<PathBuf>("patch-file").unwrap();
            let output = sub_m.get_one::<PathBuf>("output-file").unwrap();

            if Path::new(output).try_exists().is_ok() {
                fs::remove_file(output)?;
            }

            let mut rom = OpenOptions::new().read(true).write(true).open(input)?;
            let mut new_file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(output)?;
            let mut buf: Vec<u8> = vec![];
            rom.read_to_end(&mut buf)?;
            let ptc = fs::read(patch)?;

            new_file.write_all(&decode(&ptc, &buf).unwrap())?;

            rom.sync_all()?;
            new_file.sync_all()?;
        }
        _ => unreachable!(),
    }
    Ok(())
}
