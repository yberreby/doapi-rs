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

        let req = RequestParams {
            method: Method::Post,
            relative_url: DROPLETS_BASE_PATH.into(),
            body: Some(body),
        };

        self.client.send(req)
    }
}
