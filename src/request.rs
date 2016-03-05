use common::*;
use serde;

/// This is what we use to construct the HTTP request sent to DO.
/// A request template, if you wish.
pub struct DoRequest {
    pub method: Method,
    pub relative_url: String,
    pub body: Option<String>,
}

/// A droplet, as used in a request.
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
    /// An image slug.
    Slug(String),
    /// An image ID (can be private or public).
    Id(i64),
}

impl serde::Serialize for Image {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: serde::Serializer
    {
        match *self {
            Image::Slug(ref s) => serializer.serialize_str(&s),
            Image::Id(id) => serializer.serialize_i64(id),
        }
    }
}
