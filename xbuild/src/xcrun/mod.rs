use std::process::ExitStatus;

pub struct Xcrun {}

impl Xcrun {
    fn swift_runtime_paths(target: &str) -> Result<Vec<std::path::PathBuf>, String> {
        let mut c = Self::build_cmd();

        c.args(["swiftc", "-target", target, "-print-target-info"]);
        let output = c.output().map_err(|e| e.to_string())?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).into_owned());
        }

        let info: serde_json::Value =
            serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;

        Ok(info["paths"]["runtimeLibraryPaths"]
            .as_array()
            .ok_or_else(|| "swiftc output has no runtimeLibraryPaths".to_string())?
            .iter()
            .filter_map(|p| p.as_str())
            .map(std::path::PathBuf::from)
            .collect())
    }

    fn ios_sdk_path(sdk: &str) -> Result<std::path::PathBuf, String> {
        let output = Self::build_cmd()
            .args(["--sdk", sdk, "--show-sdk-path"])
            .output()
            .map_err(|e| e.to_string())?;

        if !output.status.success() {
            return Err(String::from_utf8(output.stderr).unwrap());
        }

        Ok(std::path::PathBuf::from(
            String::from_utf8_lossy(&output.stdout).trim(),
        ))
    }

    pub fn build_swift_code(
        &self,
        files: Vec<std::path::PathBuf>,
        output: &std::path::Path,
        target: &str,
        sdk: &str,
    ) -> Result<Vec<std::path::PathBuf>, String> {
        let sdk = Self::ios_sdk_path(sdk)?;

        let mut c = Self::build_cmd();
        c.args([
            "swiftc",
            "-target",
            target,
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
        let output = c.output().map_err(|e| e.to_string())?;
        if !output.status.success() {
            return Err(String::from_utf8(output.stderr).unwrap());
        }
        Self::swift_runtime_paths(target)
    }

    pub fn build_cmd() -> std::process::Command {
        std::process::Command::new("xcrun")
    }

    pub fn new() -> Result<Self, String> {
        let mut command = Self::build_cmd();

        // Check that xcrun can actually be executed.
        command.args(["--version"]);

        let status = command.status();
        if let Ok(stat) = status {
            if stat.success() {
                return Ok(Self {});
            }
        }
        Err("xcrun not found".to_string())
    }
}
