mod parse;

use parse::{parse, Command, ParseError};
use std::collections::VecDeque;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn read_to_newline(stream: &mut TcpStream) -> String {
    let mut string = String::new();
    let mut buf = [0];

    loop {
        stream.read(&mut buf).unwrap();
        if buf[0] == b'\n' {
            break;
        }

        string.push(buf[0] as char);
    }

    string
}

fn main() {
    let mut data: VecDeque<String> = VecDeque::new();
    let l = TcpListener::bind("127.0.0.1:9001").unwrap();

    for stream in l.incoming() {
        let mut stream = stream.unwrap();

        // There's no nice function to read to \n in libstd so we write our own here
        let buf = read_to_newline(&mut stream);

        // Use the parse function we wrote earlier!
        let res = parse(&buf);

        match res {
            // For a publish command push all the data
            Ok(Command::Pub(v)) => v.into_iter().for_each(|x| data.push_back(x)),

            // For a get command pop enough elements and write them
            Ok(Command::Get(num)) => (0..num)
                .fold(Vec::new(), |mut vec, _| {
                    vec.push(data.pop_front());
                    vec
                })
                .iter()
                .for_each(|i| {
                    stream
                        .write(i.as_ref().unwrap().as_bytes())
                        .map(|_| ())
                        .unwrap()
                }),

            // All errors send a reply
            Err(e) => stream
                .write(format!("<error: {:?}>", e).as_bytes())
                .map(|_| ())
                .unwrap(),
        }
    }
}
