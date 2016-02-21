#[derive(Deserialize, Debug)]
pub struct Droplet {
    pub id: f64,
    pub name: String,
    pub memory: f64,
    pub vcpus: f64,
    pub disk: f64,
    pub locked: bool,
    pub status: String,
    pub kernel: Option<Kernel>,
    pub created_at: String,
    pub features: Vec<String>,
    pub backup_ids: Vec<Option<String>>,
    pub next_backup_window: Option<Backup>,
    pub snapshot_ids: Vec<Option<String>>,
    pub image: Image,
    pub region: Region,
    pub size: Size,
    pub size_slug: String,
    pub networks: Networks,
}

#[derive(Deserialize, Debug)]
pub struct Kernel {
    pub id: f64,
    pub name: String,
    pub version: String,
}


#[derive(Deserialize, Debug)]
pub struct Backup {
    pub id: f64,
    pub name: String,
    #[serde(rename = "type")]
    pub b_type: String,
    pub distribution: String,
    pub slug: Option<String>,
    pub public: bool,
    pub regions: Vec<String>,
    pub min_disk_size: f64,
}


#[derive(Deserialize, Debug, Clone)]
pub struct Network {
    pub ip_address: String,
    pub netmask: String,
    pub gateway: String,
    #[serde(rename = "type")]
    pub network_type: String,
}

#[derive(Deserialize, Debug)]
pub struct Networks {
    pub v4: Vec<Option<Network>>,
    pub v6: Vec<Option<Network>>,
}


#[derive(Deserialize, Debug)]
pub struct Image {
    id: f64,
    name: String,
    distribution: String,
    slug: Option<String>,
    public: bool,
    regions: Vec<String>,
    created_at: String,
    min_disk_size: f64,
    #[serde(rename = "type")]
    image_type: String,
}

#[derive(Deserialize, Debug)]
pub struct Size {
    slug: String,
    memory: f64,
    vcpus: f64,
    disk: f64,
    transfer: f64,
    price_monthly: f64,
    price_hourly: f64,
    regions: Vec<String>,
    available: bool,
}

#[derive(Deserialize, Debug)]
pub struct Region {
    name: String,
    slug: String,
    sizes: Vec<String>,
    features: Vec<String>,
    available: bool,
}
