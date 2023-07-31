# anisette-server
anisette-server is a simple program to streamline the management of the docker crate: [alt_anisette_server](https://hub.docker.com/r/nyamisty/alt_anisette_server)

## WHY❓
Running an anisette is currently required to run alt-server on your linux machine. This is due to the current default anisette-server being unmaintained and unfunctional thus leading to alt-server not working. It also provides peace of mind due to its self-hosted nature if you run it on your own machine. However, running the docker crate utilising the docker cli can sometimes be confusing and time consuming, therefore leading to the creation of this tool to speed up the process.

## RUN🏃
#### WITH CARGO🚚:
```bash
git clone https://github.com/max-amb/alt-server-anisette-server.git && cd alt-server-anisette-server
cargo run # To build and run 
```
#### WITHOUT CARGO⛔🚚:
- go to releases and download the latest binary
```bash
chmod +x {YOUR BINARY}
./{YOUR BINARY}
```
## BUILD👷:
```bash
git clone https://github.com/max-amb/alt-server-anisette-server.git && cd alt-server-anisette-server
cargo build # To just build a binary
```

## DOCKER🐳:
This package requires a rootless docker install, there are many ways to achieve this on most distros however I will link the main distros and their respective guides here:
- [Arch](https://wiki.archlinux.org/title/Docker#Rootless_Docker_daemon)
- [Fedora](https://developer.fedoraproject.org/tools/docker/docker-installation.html)
- [Nixos](https://nixos.wiki/wiki/Docker#Rootless_docker)
- [Ubuntu and Debian](https://linuxhandbook.com/rootless-docker/)

## FAQS🙋
Q: Does it require being run as root?

A: No, due to you having a rootless docker install the program can interact with your docker daemon without root privileges, however if you are not running a rootless docker install then it may require root privileges. That being said, running it as root should not break functionality and if it does please create an issue.

Q: Why does it ask me to set an environment variable after I start it?

A: This is to inform alt-server that we are running our own anisette-server and that it should use our one instead of the default one. Please note that the environment variables are wiped when you elevate priveleges inside a shell, e.g through ```sudo su```, thus you will have to set the environment variable in the same shell that you intend to run alt-server on - perhaps being a root shell.

Q: I have encountered a bug/The program crashed/Other issues - what should I do?

A: In the case where the program is faulty then I would appreciate the opening of an issue to inform me of the bug, this will allow me to fix it in the future and thus prevent the problem from happening for you and future users. Also if you are able to provide a fix - in the form of a pr - then I would be glad to merge it!
