use beanstalkd_parser::response::{Response, convert_response};
use config::Config;
use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

use beanstalkd_parser::parse_command;

/// main entrypoint for the beanstalkd server
fn main() {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    let config = get_config();
    start_tcp_server(config);
}

/// gets configuration from the config.toml file
fn get_config() -> Config {
    Config::builder()
        .add_source(config::File::with_name("config.toml"))
        .build()
        .unwrap()
}

/// starts the TCP server and listens for incoming connections
fn start_tcp_server(config: Config) {
    let host = config.get("host").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = config
        .get("tcp_port")
        .unwrap_or_else(|_| "11300".to_string());
    log::info!("Starting TCP server on {}:{}", host, port);
    let listener = TcpListener::bind(format!("{}:{}", host, port)).unwrap();
    for stream in listener.incoming() {
        let _stream = stream.unwrap();
        handle_connection(_stream);
    }
}

/// handles an incoming TCP connection, reads the request, parses it, and sends a response back to the client
fn handle_connection(mut stream: TcpStream) {
    log::info!("Handling connection from {}", stream.peer_addr().unwrap());
    let buf_reader = BufReader::new(&mut stream);
    let request = buf_reader
        .lines()
        .map(|line| line.unwrap())
        .take_while(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\r\n")
        + "\r\n";
    log::info!(target:"server::tcp", "Received request:\n{}", request);
    match parse_command(request.as_str()) {
        Ok((_, cmd)) => {
            log::info!(target:"server::tcp", "Parsed command: {:?}", cmd);
            stream
                .write((convert_response(Response::Inserted { id: 1 }) + "\r\n").as_bytes())
                .unwrap();
            handle_connection(stream);
            // Here you would handle the command and send a response back to the client
        }
        Err(e) => {
            log::error!(target:"server::tcp", "Failed to parse command: {:?}", e);
            // Here you would send an error response back to the client
            stream
                .write((convert_response(Response::InvalidCommand) + "\r\n").as_bytes())
                .unwrap();
            handle_connection(stream);
        }
    }
}
