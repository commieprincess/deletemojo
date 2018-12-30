use std::fs::File;
use std::io::{copy, Read, Seek, SeekFrom};
use std::path::PathBuf;
use std::process;

use regex::Regex;
use structopt::StructOpt;

fn main() {
    if let Err(e) = run() {
        println!("Error: {}", e);
        process::exit(1);
    }
}

fn run() -> Result<(), &'static str> {
    let opt = Options::from_args();

    let script: String = String::from_utf8_lossy(
        &File::open(&opt.installer_path)
            .map_err(|_| "could not open installer file")?
            .bytes()
            .take_while(|v| match *v {
                Ok(v) => v != 0x1f,
                Err(_) => false,
            })
            .map(|v| v.unwrap())
            .collect::<Vec<u8>>(),
    )
    .into_owned();

    let filesize_pattern = Regex::new(r#"filesizes="(\d+)""#).unwrap();
    let filesize_match = match filesize_pattern.captures(&script) {
        Some(v) => v,
        None => return Err("no filesize match"),
    }
    .get(1)
    .unwrap()
    .as_str();
    let filesize: usize = str::parse(filesize_match).unwrap();

    let mut downloaded_file =
        File::open(&opt.installer_path).map_err(|_| "could not open installer file")?;
    downloaded_file
        .seek(SeekFrom::Start((script.len() + filesize) as u64))
        .map_err(|_| "failed to seek in file")?;

    copy(
        &mut downloaded_file,
        &mut File::create("game.tar.gz").map_err(|_| "could not create new file")?,
    )
    .map_err(|_| "could not copy data to new file")?;

    Ok(())
}

#[derive(StructOpt)]
struct Options {
    #[structopt(name = "DOWNLOADED_FILE", parse(from_os_str))]
    installer_path: PathBuf,
}
