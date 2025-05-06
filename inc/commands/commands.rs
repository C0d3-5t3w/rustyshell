use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Serialize, Deserialize)]
pub enum CommandType {
    Shell,
    Exit,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandRequest {
    pub cmd_type: CommandType,
    pub command: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandResponse {
    pub success: bool,
    pub output: String,
    pub error: String,
}

pub fn execute_shell_command(command: &str) -> CommandResponse {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", command])
            .output()
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()
    };

    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            CommandResponse {
                success: output.status.success(),
                output: stdout,
                error: stderr,
            }
        }
        Err(e) => CommandResponse {
            success: false,
            output: String::new(),
            error: format!("Failed to execute command: {}", e),
        },
    }
}
