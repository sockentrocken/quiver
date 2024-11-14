/*
* ================================================================
* utility.rs
* ================================================================
*/

pub mod file {
    pub fn write<T: AsRef<[u8]>>(path: &str, content: T) -> Result<(), String> {
        std::fs::write(path, content).map_err(|_| format!("Could not write \"{path}\"."))?;
        Ok(())
    }
}

pub mod folder {
    pub fn write(path: &str) -> Result<(), String> {
        std::fs::DirBuilder::new()
            .create(path)
            .map_err(|_| format!("Cannot over-write \"{path}\"."))?;
        Ok(())
    }
}
