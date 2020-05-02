# Common tasks in a language

## Installation of Rust + Cargo 

https://doc.rust-lang.org/cargo/getting-started/installation.html

Downloading the installer from https://win.rustup.rs/

## Creating a project

```
cargo new hello_world --bin
cd .\hello_world\
cargo build
.\target\debug\hello_world.exe
cargo run
```

## Redis

[dependencies.redis]
version = "*"

## Postgresql

[dependencies]
postgres = "*"

I did have an issue with the prepared statement though. 

## RabbitMQ

```
docker run -d --hostname rabbitmq --name rabbit-1 -p 5672:5672 rabbitmq:3.7.18 
```
[dependencies]
amiquip = "0.3.3"

I'm using the examples from the repository 
https://github.com/jgallagher/amiquip/tree/master/examples

To have multiple programs inside one cargo, I had to put them inside the src\bin folder and then run them with: 
cargo run --bin cli-send
cargo run --bin cli-recieve

or 

cargo build --bin cli-send
cargo build --bin cli-receive

## RestServer + Client
