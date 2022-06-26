#![allow(unused)]

use clap::Parser;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

#[derive(Parser)]
struct Cli {
    #[clap(long, short = 'p')]
    pattern: String,
    #[clap(long, short = 't')]
    file_type: Option<String>,
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> std::io::Result<()> {
    let args = Cli::parse();
    println!("pattern {:?}", args.pattern);
    println!("type {:?}", args.file_type);
    println!("path {:?}", args.path);
    let path = args.path;
    let images_vec = vec!["jpeg", "jpg", "png"];
    // let mut valid_file_path: Vec<PathBuf> = Vec::new();
    if std::fs::read_dir(&path).is_ok() {
        let files_in_dir = std::fs::read_dir(&path).unwrap();
        for file in files_in_dir {
            if file.is_ok() {
                let file_path = file.unwrap().path();
                let extension = Path::new(&file_path).extension().and_then(OsStr::to_str);
                match extension {
                    Some(ext) => {
                        if images_vec.contains(&ext) {
                            println!("{:?}", file_path);
                            // valid_file_path.push(file_path);
                        }
                    }
                    None => {}
                }
            }
        }
        Ok(())
    } else {
        println!("{:?} is not a directory", path);
        Ok(())
    }
}
