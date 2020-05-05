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


```
docker run -d -p 5432:5432 --name pdgb-1 -e POSTGRES_USER=docker -e POSTGRES_PASSWORD=XdccDa85_JK -e POSTGRES_DB=docker postgres 
```

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

I'll be using the Actix-web framework to serve a Rest Repository. And using [Romaric P.'s Create a blazingly fast REST API in Rust](https://docs.qovery.com/guides/tutorial/create-a-blazingly-fast-api-in-rust-part-1/) tutorial as a reference. 

[dependencies]
actix-rt = "1.1.0"
actix-web = "3.0.0"
env_logger = "0.7.1"
// Serde used here for the json serialisation but supports many formats   https://serde.rs/
serde = "1.0.106"
serde_json = "1.0.51"
// time library https://docs.rs/chrono/0.4.11/chrono/
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "0.6", features = ["serde", "v4"] }

Instead of storing the Events in a database or in Redis, I was trying to store them inside the Rust application, in a globally accessible variable. For that I am using a HashMap that is declared with lazy_static to make it accessible as a global variable from multiple threads. I then have to Mutex it to make sure the access is single threaded.
This requires the crate lazy_static: 

References: 

- https://doc.rust-lang.org/std/collections/struct.HashMap.html
- https://crates.io/crates/lazy_static


### Testing 

curl -X DELETE http://localhost:9090/events/50c2ee4d-1679-46a4-afd8-3fcb95fd0d6f

curl -H "Content-Type: application/json" -X POST -d {\"name\":\"MyEventFromPOST\"} http://localhost:9090/events


## GUI

have to clone it out and target the 1.0-prerelease branch! 
https://github.com/gabdube/native-windows-gui

[dependencies]
native-windows-gui = { git = "https://github.com/gabdube/native-windows-gui", branch = "1.0-prerelease" }
native-windows-derive =  { git = "https://github.com/gabdube/native-windows-gui", branch = "1.0-prerelease" }