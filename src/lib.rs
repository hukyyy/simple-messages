use std::{
    error::Error,
    fmt::Display,
    io::{self, Read, Write},
    net::TcpStream,
};

const MAX_MESSAGE_SIZE: usize = 512;

/// Represents a server command
/// - `Connect` sends over the `User` struct to establish a connection
/// - `Msg` sends over a `Message` struct to be forwarded to another user
#[derive(Debug)]
pub enum Command {
    Connect(User),
    Msg(Message),
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::Connect(user) => write!(f, "Connect|{}", user.0),
            Command::Msg(m) => write!(f, "Msg|{}|{}|{}", m.sender, m.receiver, m.message),
        }
    }
}

/// Returned when a valid `Command` is failed to be parsed from a stream.
#[derive(Debug)]
pub struct CommandParseError;

impl Display for CommandParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to parse command from stream")
    }
}

impl Error for CommandParseError {}

#[derive(Debug)]
pub struct Message {
    pub sender: String,
    pub receiver: String,
    pub message: String,
}

/// Represents a user. The only field contains the user's unique username.
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct User(pub String);

impl User {
    pub fn new(username: String) -> Self {
        User(username)
    }
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Sends a `Command` over the given `stream`
pub fn send_command(command: Command, stream: &mut TcpStream) -> io::Result<()> {
    let message = command.to_string();
    let _ = stream.write(message.as_bytes())?;
    Ok(())
}

/// Decodes a `Command` from the given `stream`.
///
/// # Errors
/// - `Utf8Error` if decoding the message from the stream fails.
/// - `CommandParseError` if extracting the command from the message fails.
pub fn receive_command(stream: &mut TcpStream) -> Result<Command, Box<dyn Error>> {
    let mut bytes = [0; MAX_MESSAGE_SIZE];
    let bytes_read = stream.read(&mut bytes)?;
    let message: String = str::from_utf8(&bytes[..bytes_read])?.to_owned();
    let command = match message.split("|").next().unwrap() {
        "Msg" => {
            let fields: Vec<&str> = message.split("|").collect();
            Command::Msg(Message {
                sender: fields.get(1).unwrap().to_string(),
                receiver: fields.get(2).unwrap().to_string(),
                message: fields.get(3).unwrap().to_string(),
            })
        }
        "Connect" => Command::Connect(User::new(
            message
                .split("|")
                .collect::<Vec<&str>>()
                .get(1)
                .unwrap()
                .to_string(),
        )),
        _ => {
            return Err(Box::new(CommandParseError));
        }
    };

    Ok(command)
}
