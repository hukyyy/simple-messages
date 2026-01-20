use std::io;
use std::net::TcpStream;

use messages::{Command, User, send_command};

fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    let user = User::new(String::from("Elliot"));

    send_command(Command::Connect(user), &mut stream)?;

    // TODO: actually implement sending messages over the stream

    Ok(())
}
