/*
* ================================================================
* build.rs
* ================================================================
*/

use std::fs::File;
use std::io::{self, BufRead, BufWriter, Write};
use std::path::Path;

fn main() {
    let meta = File::create("src/asset/meta.lua").unwrap();
    let base = File::create("src/asset/base.lua").unwrap();
    let mut meta = BufWriter::new(meta);
    let mut base = BufWriter::new(base);
    let mut meta_write = false;
    let mut base_write = false;

    meta.write(b"---@meta\n\n").unwrap();

    for file in std::fs::read_dir("src/system/").unwrap() {
        if let Ok(lines) = read_lines(file.unwrap().path().to_str().unwrap()) {
            for line in lines.flatten() {
                let copy = line.clone();
                let line = line.trim();

                if line.starts_with("*/") {
                    if meta_write {
                        meta.write(b"\n").unwrap();
                    }

                    if base_write {
                        base.write(b"\n").unwrap();
                    }

                    meta_write = false;
                    base_write = false;
                }

                if meta_write {
                    meta.write(line.as_bytes()).unwrap();
                    meta.write(b"\n").unwrap();
                }

                if base_write {
                    base.write(copy.as_bytes()).unwrap();
                    base.write(b"\n").unwrap();
                }

                if line.starts_with("/* meta") {
                    meta_write = true;
                }

                if line.starts_with("/* base") {
                    base_write = true;
                }
            }
        }
    }
}

fn read_lines<P>(metaname: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let meta = File::open(metaname)?;
    Ok(io::BufReader::new(meta).lines())
}
