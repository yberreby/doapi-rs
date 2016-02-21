use common::*;
use serde;

pub struct RequestParams {
    pub method: Method,
    pub relative_url: String,
    pub body: Option<String>,
}

/// A Droplet request.
#[derive(Debug, Serialize)]
pub struct Droplet {
    pub name: String,
    pub region: String,
    pub size: String,
    // DO also supports numeric IDs, for private images (snapshots?), as well
    // as slugs for public images.
    pub image: Image,
    pub ssh_keys: Option<Vec<String>>,
    pub backups: bool,
    pub ipv6: bool,
    pub private_networking: bool,
    pub user_data: Option<String>,
}

#[derive(Debug)]
pub enum Image {
    /// A public image slug.
    Public(String),
    /// A private image ID.
    Private(i64),
}

impl serde::Serialize for Image {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: serde::Serializer
    {
        match *self {
            Image::Public(ref s) => serializer.visit_str(&s),
            Image::Private(id) => serializer.visit_i64(id),
        }
    }
}
