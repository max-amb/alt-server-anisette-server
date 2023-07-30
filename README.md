# alt-server-anisette-server
alt-server-anisette-server is a simple program to streamline the management of the docker crate nyamisty/alt_anisette_server which is currently required to run alt-server on your linux machine.

It does not need to be run as root however running it as root will not break things. It creates docker containers as the root user.

## BUILD/RUN:
```rust
git clone https://github.com/max-amb/alt-server-anisette-server.git && cd alt-server-anisette-server
cargo build # To just build a binary
cargo run # To build then run it
```



