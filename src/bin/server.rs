use std::collections::HashMap;
use std::error::Error;
use std::net::{TcpListener, TcpStream};

use messages::{Command, User, receive_command};

fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    let mut users: HashMap<User, TcpStream> = HashMap::new();

    loop {
        match listener.accept() {
            Ok((mut stream, _)) => match receive_command(&mut stream) {
                Ok(command) => match command {
                    Command::Connect(user) => {
                        println!("Accepting connection request from {user}");
                        users.insert(user, stream);
                        // TODO: handle further requests
                    }
                    _ => eprintln!("User didn't send a `Connect` command! Disconnecting."),
                },
                Err(e) => eprintln!("{e}"),
            },
            Err(e) => eprintln!("Failed to accept connection: {e}"),
        }
    }
}
