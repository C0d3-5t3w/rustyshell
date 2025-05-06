use std::env;

mod config {
    pub use crate::inc::config::*;
}
mod connection {
    pub use crate::inc::connection::*;
}
mod commands {
    pub use crate::inc::commands::*;
}
mod server {
    pub use crate::inc::server::*;
}
mod client {
    pub use crate::inc::client::*;
}

#[path = "../inc/config/mod.rs"]
mod inc_config;
#[path = "../inc/connection/mod.rs"]
mod inc_connection;
#[path = "../inc/commands/mod.rs"]
mod inc_commands;
#[path = "../inc/server/mod.rs"]
mod inc_server;
#[path = "../inc/client/mod.rs"]
mod inc_client;

mod inc {
    pub mod config {
        pub use crate::inc_config::*;
    }
    pub mod connection {
        pub use crate::inc_connection::*;
    }
    pub mod commands {
        pub use crate::inc_commands::*;
    }
    pub mod server {
        pub use crate::inc_server::*;
    }
    pub mod client {
        pub use crate::inc_client::*;
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} [client|server] [config_path]", args[0]);
        std::process::exit(1);
    }
    
    let mode = &args[1];
    let config_path = if args.len() > 2 {
        &args[2]
    } else {
        "pkg/config.yaml"
    };
    
    let config = config::Config::from_file(config_path)?;
    
    match mode.as_str() {
        "server" => {
            let mut server = server::Server::new(config);
            server.run()?;
        }
        "client" => {
            let mut client = client::Client::new(config);
            client.run()?;
        }
        _ => {
            eprintln!("Invalid mode: {}. Use 'client' or 'server'", mode);
            std::process::exit(1);
        }
    }
    
    Ok(())
}
