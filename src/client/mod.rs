use common::*;

mod droplets;
pub use self::droplets::DropletsService;

/// A DigitalOcean client.
pub struct Client {
    token: String,
}

impl Client {
    /// Create a new Client with the given API token.
    pub fn with_token<T: Into<String>>(tok: T) -> Client {
        Client { token: tok.into() }
    }

    pub fn droplets(&mut self) -> DropletsService {
        DropletsService::new(self)
    }

    fn send<T>(&mut self, req: &RequestBuilder) -> DoResult<T>
        where T: ::serde::Deserialize
    {
        unimplemented!()
    }
}
