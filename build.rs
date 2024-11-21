use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

//================================================================

/// responsible for writing the Lua documentation as well as the standard library (Vector2, Vector3, Box2, Box3, etc.) to the asset/ folder.
/// it will later be built with the engine, so that it may dump it to a newly made module.
fn main() {
    // this SHOULD match the engine's internal meta/base file file-name.
    let meta_name = "meta.lua";
    let base_name = "base.lua";

    // this SHOULD match the engine's path for the Rust-Lua API.
    let system_path = "src/system/";

    //================================================================

    // create the meta.lua/base.lua file in the asset folder.
    let meta = File::create(format!("src/asset/module/{meta_name}"))
        .unwrap_or_else(|_| panic!("build.rs: Could not create {meta_name} file."));
    let base = File::create(format!("src/asset/module/{base_name}"))
        .unwrap_or_else(|_| panic!("build.rs: Could not create {base_name} file."));
    let mut meta = BufWriter::new(meta);
    let mut base = BufWriter::new(base);
    let mut meta_write = false;
    let mut base_write = false;

    //================================================================

    // write the ---@meta header for meta.lua.
    meta.write_all(b"---@meta\n\n").unwrap();

    // read every file in the API directory.
    for file in std::fs::read_dir(system_path).unwrap() {
        // convert to string.
        let file = file.expect("build.rs: Could not unwrap file.").path();
        let file = file
            .to_str()
            .expect("build.rs: Could not convert file path to string.");

        // open file, use bufReader.
        let source = File::open(file)
            .unwrap_or_else(|_| panic!("build.rs: Could not open file \"{meta_name}\"."));
        let source = BufReader::new(source).lines();

        // for every line in the file...
        for line in source.map_while(Result::ok) {
            // keep the original version of the line, with trailing white-space and all, for base.lua.
            let copy = line.clone();

            // trim the line for comparison with /* marker.
            let line = line.trim();

            // END marker. write line.
            if line.starts_with("*/") {
                // write meta.lua line.
                if meta_write {
                    meta.write_all(b"\n").unwrap();
                }

                // write base.lua line.
                if base_write {
                    base.write_all(b"\n").unwrap();
                }

                // do not write any other line until reaching another START marker for either file.
                meta_write = false;
                base_write = false;
            }

            // can write to meta.lua, write!
            if meta_write {
                meta.write_all(line.as_bytes()).unwrap();
                meta.write_all(b"\n").unwrap();
            }

            // can write to base.lua, write!
            if base_write {
                base.write_all(copy.as_bytes()).unwrap();
                base.write_all(b"\n").unwrap();
            }

            // START meta marker, begin writing to meta.lua.
            if line.starts_with("/* meta") {
                meta_write = true;
            }

            // START base marker, begin writing to base.lua.
            if line.starts_with("/* base") {
                base_write = true;
            }
        }
    }
}
