use request;
use response;

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
