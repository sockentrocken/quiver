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

    pub fn read(path: &str) -> Result<String, String> {
        std::fs::read_to_string(path).map_err(|e| e.to_string())
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

pub fn panic_window(message: &str) {
    rfd::MessageDialog::new()
        .set_level(rfd::MessageLevel::Error)
        .set_title("Fatal Error")
        .set_description(message)
        .set_buttons(rfd::MessageButtons::Ok)
        .show();
}
