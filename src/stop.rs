use crate::already_made;
use dockers::Container;
pub fn stop() {
    let cont = Container { Id: "alt_anisette_server".to_string(), ..Default::default() }; 
    if already_made() {
        cont.kill().expect("container was not running");
    } else {
        println!("Container was not made thus was not running , try the command -> alt-store-anisette-server start");
        std::process::exit(1);
    }
}

