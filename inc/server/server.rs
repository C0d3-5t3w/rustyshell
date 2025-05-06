use crate::config::Config;
use crate::connection::Connection;
use crate::commands::{CommandRequest, CommandResponse, CommandType};
use std::io::{self, Write, BufRead};
use std::net::{TcpListener, TcpStream};
use serde_json;

pub struct Server {
    config: Config,
    listener: Option<TcpListener>,
}

impl Server {
    pub fn new(config: Config) -> Self {
        Server {
            config,
            listener: None,
        }
    }

    pub fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let addr = format!("{}:{}", self.config.server.host, self.config.server.port);
        let listener = TcpListener::bind(&addr)?;
        println!("Server started on {}", addr);
        self.listener = Some(listener);
        
        Ok(())
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.listener.is_none() {
            self.start()?;
        }
        
        let listener = self.listener.as_ref().unwrap();
        
        println!("Waiting for client to connect...");
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("Client connected: {}", stream.peer_addr()?);
                    self.handle_client(stream)?;
                }
                Err(e) => {
                    eprintln!("Connection failed: {}", e);
                }
            }
        }
        
        Ok(())
    }
    
    fn handle_client(&self, stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
        let mut connection = Connection::new(stream);
        let stdin = io::stdin();
        let mut stdin_lock = stdin.lock();
        
        loop {
            print!("rustyshell> ");
            io::stdout().flush()?;
            
            let mut input = String::new();
            stdin_lock.read_line(&mut input)?;
            let input = input.trim();
            
            if input == "exit" || input == "quit" {
                let exit_cmd = CommandRequest {
                    cmd_type: CommandType::Exit,
                    command: String::new(),
                };
                let cmd_str = serde_json::to_string(&exit_cmd)?;
                connection.send(cmd_str.as_bytes())?;
                break;
            }
            
            let shell_cmd = CommandRequest {
                cmd_type: CommandType::Shell,
                command: input.to_string(),
            };
            
            let cmd_str = serde_json::to_string(&shell_cmd)?;
            connection.send(cmd_str.as_bytes())?;
            
            // Get response
            let mut buf = [0; 4096];
            let read = connection.receive(&mut buf)?;
            if read == 0 {
                println!("Client closed connection");
                break;
            }
            
            let resp_str = std::str::from_utf8(&buf[0..read])?;
            let response: CommandResponse = serde_json::from_str(resp_str)?;
            
            if !response.output.is_empty() {
                println!("{}", response.output);
            }
            
            if !response.error.is_empty() {
                eprintln!("Error: {}", response.error);
            }
        }
        
        Ok(())
    }
}
