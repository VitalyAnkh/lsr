#[macro_use]
extern crate structopt;
use chrono::{DateTime, Local};
use libc::{S_IRGRP, S_IROTH, S_IRUSR, S_IWGRP, S_IWOTH, S_IWUSR, S_IXGRP, S_IXOTH, S_IXUSR};
use std::error::Error;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::process;
use structopt::StructOpt;
#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(default_value = ".", parse(from_os_str))]
    path: PathBuf,
}
fn main() {
    let opt = Opt::from_args();
    if let Err(ref e) = run(&opt.path) {
        println!("{}", e);
        process::exit(1);
    }
}

fn run(dir: &PathBuf) -> Result<(), Box<Error>> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let file_name = entry
                .file_name()
                .into_string()
                .or_else(|f| Err(format!("Invalid entry: {:?}", f)))?;
            let metadata = entry.metadata()?;
            let mode = metadata.permissions().mode();

            let size = metadata.len();
            let modified: DateTime<Local> = DateTime::from(metadata.modified()?);
            println!(
                "{} {:>5} {} {}",
                parse_permissions(mode as u16),
                size,
                modified.format("%_d %b %H:%M").to_string(),
                file_name
            );
        }
    }
    Ok(())
}

fn parse_permissions(mode: u16) -> String {
    let user = triplet(mode, S_IRUSR as u16, S_IWUSR as u16, S_IXUSR as u16);
    let group = triplet(mode, S_IRGRP as u16, S_IWGRP as u16, S_IXGRP as u16);
    let other = triplet(mode, S_IROTH as u16, S_IWOTH as u16, S_IXOTH as u16);
    [user, group, other].join("")
}

fn triplet(mode: u16, read: u16, write: u16, execute: u16) -> String {
    match (mode & read, mode & write, mode & execute) {
        (0, 0, 0) => "---",
        (_, 0, 0) => "r--",
        (0, _, 0) => "-w-",
        (0, 0, _) => "--x",
        (_, _, 0) => "rw-",
        (_, 0, _) => "r-x",
        (0, _, _) => "_wx",
        (_, _, _) => "rwx",
    }
    .to_string()
}
