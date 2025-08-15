use std::process::{Command, Stdio};
use crate::errors::AppResult; 

pub fn run_command(cmd: &str, args: &[&str]) -> AppResult<()> {
    let status = Command::new(cmd)
        .args(args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?;

    if !status.success() {
        return Err(crate::errors::errors::FTVError::FfmpegFailed(format!(
            "Command {:?} failed with exit code {:?}",
            cmd,
            status.code()
        )));
    }

    Ok(())
}