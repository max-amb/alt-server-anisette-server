use dockers::{
    containers::{ContainerConfig, HostConfig, PortBinding},
    Container,
};
use std::collections::HashMap;
use std::env;
use crate::{ 
    stop::stop,
    rm::rm,
    start::start,
};
mod stop; mod rm; mod start;

const HELP_MESSAGE: &str = "\nUSAGE: anisette-server <start/stop/rm/help> 
    start -> starts (or creates it if its not already made) the anisette server container and checks if the environment variables are set correctly 
    stop -> kills/stops the anisette server container
    rm -> deletes the image and container from docker 
    help -> displays this help message
";
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 { println!("anisette-server: Command was not provided, try 'anisette-server help' for more information"); std::process::exit(127);}
    else if args.len() > 2 { println!("anisette-server: Too many commands provided, try 'anisette-server help' for more information"); std::process::exit(127);}
    else if args[1] == "help" { println!("{HELP_MESSAGE}"); std::process::exit(1);}
    else if args[1] == "start" { start(); }
    else if args[1] == "stop" { stop(); }
    else if args[1] == "rm" { rm(); }
    else { println!("anisette-server: No valid command provided, try 'anisette-server help' for more information"); std::process::exit(127);}
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
