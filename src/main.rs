use beanstalkd_parser::response::{Response, convert_response};
use config::Config;
use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

use beanstalkd_parser::parse_command;

fn main() {
    let config = get_config();
    start_tcp_server(config);
}
fn get_config() -> Config {
    Config::builder()
        .add_source(config::File::with_name("config.toml"))
        .build()
        .unwrap()
}

fn start_tcp_server(config: Config) {
    let port = config
        .get("tcp_port")
        .unwrap_or_else(|_| "11300".to_string());
    println!("Starting TCP server on port {}", port);
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).unwrap();
    for stream in listener.incoming() {
        let _stream = stream.unwrap();
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
                .write((convert_response(Response::Inserted { id: 1 }) + "\r\n").as_bytes())
                .unwrap();
            handle_connection(stream);
            // Here you would handle the command and send a response back to the client
        }
        Err(e) => {
            println!("Failed to parse command: {:?}", e);
            // Here you would send an error response back to the client
        }
    }
}
