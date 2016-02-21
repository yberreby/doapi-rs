extern crate doapi;

fn main() {
    let token = env!("DO_AUTH_TOKEN");
    let mut client = doapi::Client::with_token(token);

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

    let resp = client.droplets().create(&droplet_params).unwrap();

    println!("Response:\n{:?}", resp);
}
