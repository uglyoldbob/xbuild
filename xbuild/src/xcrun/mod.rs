use std::process::ExitStatus;

pub struct Xcrun {
}

impl Xcrun {
    fn ios_sdk_path() -> Result<std::path::PathBuf, String> {
        let output = Self::build_cmd()
            .args(["--sdk", "iphoneos", "--show-sdk-path"])
            .output().map_err(|e| e.to_string())?;

        if !output.status.success() {
            return Err(String::from_utf8(output.stderr).unwrap());
        }

        Ok(std::path::PathBuf::from(
            String::from_utf8_lossy(&output.stdout).trim(),
        ))
    }

    pub fn build_swift_code(&self, files: Vec<std::path::PathBuf>, output: &std::path::Path) -> Result<(), String> {
        let sdk = Self::ios_sdk_path()?;
        
        let mut c = Self::build_cmd();
        c.args([
            "swiftc",
            "-target",
            "arm64-apple-ios",
            "-sdk",
            &sdk.display().to_string(),
            "-parse-as-library",
            "-emit-object",
            "-o",
        ]);
        c.arg(&output);
        for f in files {
            c.arg(f);
        }
        let output = c
            .output().map_err(|e| e.to_string())?;
        if !output.status.success() {
            return Err(String::from_utf8(output.stderr).unwrap());
        }
        Ok(())
    }

    pub fn build_cmd() -> std::process::Command {
        std::process::Command::new("xcrun")
    }

    pub fn new() -> Result<Self, String> {
        let mut command = Self::build_cmd();

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