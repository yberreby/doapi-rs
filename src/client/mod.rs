use request;
use response;

/// A DigitalOcean client.
pub struct Client {
    token: String,
}

impl Client {
    /// Create a new Client with the given API token.
    pub fn with_token<T: Into<String>>(tok: T) -> Client {
        Client { token: tok.into() }
    }

    pub fn droplets(&self) -> DropletsService {
        DropletsService::with_token(&self.token)
    }
}



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
