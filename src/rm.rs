use crate::already_made;
use dockers::Container;
pub fn rm() {
    let cont = Container { Id: "alt_anisette_server".to_string(), ..Default::default() }; 
    if already_made() {
    cont.kill().expect("container was not running, deleting now");
    cont.remove().expect("Failed to delete container");
    } else {
        println!("Container was not made, try the command -> alt-store-anisette-server start");
        std::process::exit(1);
    }
}
