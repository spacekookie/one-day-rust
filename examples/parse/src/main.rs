#[derive(Debug)]
enum Command {
    Pub(Vec<String>),
    Get(u32),
}

#[derive(Debug)]
enum ParseError {
    UnknownCommand,
    InvalidPayload,
    Incomplete,
}

fn parse(input: &str) -> Result<Command, ParseError> {
    if input.len() == 0 {
        return Err(ParseError::Incomplete);
    }
    
    let input: String = input.into();
    match &input[0..2] {
        "PUB" => Ok(Command::Pub(
            input[3..]
                .to_owned()
                .split(",")
                .map(|s| s.to_owned())
                .collect(),
        )),
        "GET" => Ok(Command::Get(
            input[3..].parse().map_err(|_| ParseError::InvalidPayload)?,
        )),
        _ => Err(ParseError::UnknownCommand),
    }
}

fn main() {
    let cmd = parse("PUB hello, to, the, world");
    println!("{:?}", cmd);
}
