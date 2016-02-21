use request;
use response;

const DROPLETS_BASE_PATH: &'static str = "v2/droplets";

pub struct DropletsService<'tok> {
    token: &'tok str,
}

impl<'tok> DropletsService<'tok> {
    pub fn with_token(token: &str) -> DropletsService {
        DropletsService { token: token }
    }

    pub fn create(droplet_req: &request::Droplet) -> response::Droplet {

        unimplemented!()
    }
}
