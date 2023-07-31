use crate::{
   already_made,
   Container,
   HostConfig,
   ContainerConfig,
   PortBinding,
   HashMap,
   env,
};

pub fn start() {
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
