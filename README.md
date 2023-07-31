# anisette-server
anisette-server is a simple program to streamline the management of the docker crate nyamisty/alt_anisette_server which is currently required to run alt-server on your linux machine. Due to this it requires a docker install.

It does not need to be run as root however running it as root will not break things. It creates docker containers as the root user.

## RUN🏃 
#### WITHOUT CARGO:
- go to releases and download the latest binary
```bash
chmod +x {YOUR BINARY}
./{YOUR BINARY}
```
#### WITH CARGO:
```bash
git clone https://github.com/max-amb/alt-server-anisette-server.git && cd alt-server-anisette-server
cargo run # To build and run
```
## BUILD👷:
```bash
git clone https://github.com/max-amb/alt-server-anisette-server.git && cd alt-server-anisette-server
cargo build # To just build a binary
```

## DOCKER🐳:
This package requires a rootless docker install, there are man ways to achieve this on many distrobutions however I will link the main distros and their respective guides here:
- [Arch](https://wiki.archlinux.org/title/Docker#Rootless_Docker_daemon)
- [Fedora](https://developer.fedoraproject.org/tools/docker/docker-installation.html)
- [Nixos](https://nixos.wiki/wiki/Docker#Rootless_docker)
- [Ubuntu and Debian](https://linuxhandbook.com/rootless-docker/)
