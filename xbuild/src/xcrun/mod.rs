use std::process::ExitStatus;

pub struct Xcrun {}

impl Xcrun {
    pub fn new() -> Result<Self, String> {
        let mut command = std::process::Command::new("xcrun");

        // Check that xcrun can actually be executed.
        command
            .args(["--version"]);

        let status = command
            .status();
        if let Ok(stat) = status {
            if stat.success() {
                return Ok(Self {});
            }
        }
        Err("xcrun not found".to_string())
    }
}