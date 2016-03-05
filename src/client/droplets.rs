use common::*;

const DROPLETS_BASE_PATH: &'static str = "v2/droplets";

pub struct DropletsService<'client> {
    client: &'client mut Client,
}

impl<'tok> DropletsService<'tok> {
    pub fn new(client: &mut Client) -> DropletsService {
        DropletsService { client: client }
    }

    pub fn create(&mut self, droplet_req: &request::Droplet) -> DoResult<response::Droplet> {
        let body = try!(::serde_json::to_string(&droplet_req));

        let req = DoRequest {
            method: Method::Post,
            relative_url: DROPLETS_BASE_PATH.into(),
            body: Some(body), // that I used to know
        };

        self.client.send_request(req, "droplet")
    }

    /// Retrieve an individual droplet by ID.
    ///
    /// Useful if you want to check its status.
    pub fn get(&mut self, id: i64) -> DoResult<response::Droplet> {
        let req = DoRequest {
            method: Method::Get,
            // XXX - string interpolation is not ideal, is it?
            relative_url: format!("{}/{}", DROPLETS_BASE_PATH, id),
            body: None,
        };

        self.client.send_request(req, "droplet")
    }
}
