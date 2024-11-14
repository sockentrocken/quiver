/*
* Copyright (c) 2024 sockentrocken
*
* Permission to use, copy, modify, and distribute this software for any
* purpose with or without fee is hereby granted, provided that the above
* copyright notice and this permission notice appear in all copies.
*
* THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
* WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
* MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
* ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
* WHATSOEVER RESULTING FROM LOSS OF MIND, USE, DATA OR PROFITS, WHETHER
* IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING
* OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
*/

/*
* ================================================================
* build.rs
* ================================================================
*/

use std::fs::File;
use std::io::{self, BufRead, BufWriter, Write};
use std::path::Path;

fn main() {
    let file = File::create("src/asset/meta.lua").unwrap();
    let mut file = BufWriter::new(file);
    let mut test = false;

    file.write(b"---@meta\n\n").unwrap();

    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("src/system.rs") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            let line = line.trim();

            if line.starts_with("*/") {
                if test {
                    file.write(b"\n").unwrap();
                }

                test = false;
            }

            if test {
                file.write(line.as_bytes()).unwrap();
                file.write(b"\n").unwrap();
            }

            if line.starts_with("/* meta") {
                test = true;
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
