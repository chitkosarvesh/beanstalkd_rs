use beanstalkd_parser::response::{Response, convert_response};
use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

use beanstalkd_parser::parse_command;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:11300").unwrap();
    for stream in listener.incoming() {
        let _stream = stream.unwrap();
        println!("New connection!");
        handle_connection(_stream);
    }
}
fn handle_connection(mut stream: TcpStream) {
    println!("Handling connection from {}", stream.peer_addr().unwrap());
    let buf_reader = BufReader::new(&mut stream);
    let request = buf_reader
        .lines()
        .map(|line| line.unwrap())
        .take_while(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\r\n")
        + "\r\n";
    println!("Received request:\n{}", request);
    match parse_command(request.as_str()) {
        Ok((_, cmd)) => {
            println!("Parsed command: {:?}", cmd);
            stream
                .write(convert_response(Response::Inserted { id: 1 }).as_bytes())
                .unwrap();
            // Here you would handle the command and send a response back to the client
        }
        Err(e) => {
            println!("Failed to parse command: {:?}", e);
            // Here you would send an error response back to the client
        }
    }
}
