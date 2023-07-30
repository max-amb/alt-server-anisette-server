use dockers::{
    containers::{ContainerConfig, HostConfig, PortBinding},
    Container,
};
use std::collections::HashMap;
use std::env;

const HELP_MESSAGE: &str = "\nUSAGE: alt-store-anisette-server <start/stop/rm/help> 
    start -> starts (or creates it if its not already made) the anisette server container and checks if the environment variables are set correctly 
    stop -> kills/stops the anisette server container
    rm -> deletes the image and container from docker 
    help -> displays this help message

    alt-store-anisette-server is a simple program to streamline the management of the docker crate nyamisty/alt_anisette_server which is currently required to run alt-server on your linux machine";

fn main() {
    check_args();
}

fn check_args() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 { println!("Command was not provided: {}", HELP_MESSAGE); std::process::exit(127);}
    else if args.len() > 2 { println!("Too many commands provided {}", HELP_MESSAGE); std::process::exit(127);}
    else if args[1] == "help" { println!("{HELP_MESSAGE}"); std::process::exit(1);}
    else if args[1] == "start" { make_and_start(); }
    else if args[1] == "stop" { stop(); }
    else if args[1] == "rm" { remove(); }
}

fn stop() {
    let cont = Container { Id: "alt_anisette_server".to_string(), ..Default::default() }; 
    if already_made() {
        cont.kill().expect("container was not running");
    } else {
        println!("Container was not made thus was not running , try the command -> alt-store-anisette-server start");
        std::process::exit(1);
    }
}

fn remove() {
    let cont = Container { Id: "alt_anisette_server".to_string(), ..Default::default() }; 
    if already_made() {
    cont.kill().expect("container was not running, deleting now");
    cont.remove().expect("Failed to delete container");
    } else {
        println!("Container was not made, try the command -> alt-store-anisette-server start");
        std::process::exit(1);
    }
}

fn make_and_start() {
    if !already_made() {
        let mut published_ports = HashMap::new();
        let img = "nyamisty/alt_anisette_server".to_owned();

        published_ports.insert(
            "6969/tcp".to_owned(),
            vec![PortBinding {
                HostPort: "6969".to_owned(),
                ..Default::default()
            }],
        );

        let container_conf = ContainerConfig {
            Image: img.clone(),
            HostConfig: HostConfig {
                PortBindings: Some(published_ports),
                ..Default::default()
            },
            ..Default::default()
        };
        let cont = Container::new(None, Some(img)) 
            .create(Some("alt_anisette_server".to_owned()), Some(container_conf))
            .expect("Cannot create container");
        cont.start().expect("Failed to start container");
    } else {
        let cont = Container { Id: "alt_anisette_server".to_string(), ..Default::default() }; 
        cont.restart().expect("Failed to start already made container");
    }

    for (key, value) in env::vars_os() {
        if key == "ALTSERVER_ANISETTE_SERVER" && value == "http://127.0.0.1:6969" {
            println!("environment variables set up correctly");
            std::process::exit(0);
        }
    }
    println!("environment variables not set up correctly\n
            try the following command -> export ALTSERVER_ANISETTE_SERVER=http://127.0.0.1:6969")
}

fn already_made() -> bool{
    let containers = Container::list().unwrap_or(vec![]);
        println!("{:?}", containers);
    for i in containers {
        if i.Names.contains(&"/alt_anisette_server".to_string()) {
            return true;
        }
    }
    return false;
}
