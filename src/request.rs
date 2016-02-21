/// A Droplet request.
pub struct Droplet {
    pub name: String,
    pub region: String,
    pub size: String,
    pub image: String,
    pub ssh_keys: Option<Vec<String>>,
    pub backups: bool,
    pub ipv6: bool,
    pub private_networking: bool,
    pub user_data: Option<String>,
}
