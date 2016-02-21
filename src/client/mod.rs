use common::*;
use hyper;
use serde;
use serde_json;

mod droplets;
pub use self::droplets::DropletsService;

lazy_static! {
    static ref API_ROOT: Url = Url::parse("https://api.digitalocean.com").unwrap();
}

/// A DigitalOcean client.
pub struct Client {
    token: String,
    http_client: ::hyper::Client,
}

impl Client {
    /// Create a new Client with the given API token.
    pub fn with_token<T: Into<String>>(tok: T) -> Client {
        let http_client = hyper::Client::new();
        Client {
            token: tok.into(),
            http_client: http_client,
        }
    }

    pub fn droplets(&mut self) -> DropletsService {
        DropletsService::new(self)
    }

    fn send<T>(&mut self, req_params: RequestParams) -> DoResult<T>
        where T: ::serde::Deserialize
    {
        use hyper::header::{Authorization, Bearer};

        let RequestParams {
            relative_url,
            method,
            body
        } = req_params;

        let url = API_ROOT.join(&relative_url).unwrap();

        let auth_header = Authorization(Bearer { token: self.token.to_owned() });

        let resp = try!(self.http_client
                            .request(method, url)
                            .header(auth_header)
                            .send());

        serde_json::from_reader(resp).map_err(Into::into)
    }
}
