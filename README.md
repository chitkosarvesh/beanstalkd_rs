# beanstalkd_rs [![Rust](https://github.com/chitkosarvesh/beanstalkd_rs/actions/workflows/rust.yml/badge.svg)](https://github.com/chitkosarvesh/beanstalkd_rs/actions/workflows/rust.yml)

## Rust implementation of beanstalkd

### What is beanstalkd?

In short, beanstalkd is a small job queue.  
Check out the original project written in C [here](https://beanstalkd.github.io)

### Structure

```
beanstalkd_parser/ -> the beanstalkd command parser library
                |__ src/
                    |__ parsers/* -> contains all command parsers with tests
                    |__ command.rs -> command definition
                    |__ lib.rs -> main library
                    |__ response.rs  -> response definition
beanstalkd_rs/ -> the server implementation
            |__ src/
                |__ main.rs -> entrypoint of the application
config.toml -> configuration file of the project
log4rs.yml -> log configuration file of the project
```

### Protocol

Refer to the protocol documentation [here](https://raw.githubusercontent.com/beanstalkd/beanstalkd/master/doc/protocol.txt)
