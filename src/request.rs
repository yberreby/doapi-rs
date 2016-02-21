/// A Droplet request.
pub struct Droplet {
    pub name: String,
    pub region: String,
    pub size: String,
    // For now, we only support public image slugs, so this is a String.
    // But DO also supports numeric IDs, for private images (snapshots?).
    pub image: String,
    pub ssh_keys: Option<Vec<String>>,
    pub backups: bool,
    pub ipv6: bool,
    pub private_networking: bool,
    pub user_data: Option<String>,
}
