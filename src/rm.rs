use crate::already_made;
use dockers::{Container, docker::DockerApiError};
pub fn rm() {
    let cont = Container { Id: "alt_anisette_server".to_string(), ..Default::default() }; 
    if already_made() {
    let killing = || -> Result<(), DockerApiError> {
        cont.kill()?;
        Ok(())
    };
    if let Err(_err) = killing() {
        println!("Container was not running, removing now!");
    };
    cont.remove().expect("Failed to delete container");
    } else {
        println!("Container was not made, try the command -> alt-store-anisette-server start");
        std::process::exit(1);
    }
}
