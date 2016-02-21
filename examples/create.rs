extern crate doapi;
use std::thread;
use std::time::Duration;

fn main() {
    let token = env!("DO_AUTH_TOKEN");
    let mut client = doapi::Client::with_token(token);

    // Create a droplet.
    let droplet_params = doapi::request::Droplet {
        name: "doapi-test".into(),
        region: "ams2".into(),
        size: "512mb".into(),
        image: doapi::request::Image::Public("debian-8-x64".into()),
        backups: false,
        ipv6: false,
        private_networking: false,
        ssh_keys: None,
        user_data: None,
    };

    println!("Request:\n{:?}", droplet_params);

    let mut new_droplet = client.droplets().create(&droplet_params).unwrap();

    // Wait for it to be up.
    while new_droplet.status == "new" {
        println!("New droplet:\n{:?}", new_droplet);
        println!("droplet not ready yet, sleeping...");
        thread::sleep(Duration::from_secs(5));
        new_droplet = client.droplets().get(new_droplet.id).unwrap();
    }

    // It is up!
    println!("Droplet spinned up successfully.");
}
