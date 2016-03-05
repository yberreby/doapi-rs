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

    /// Send a request to DigitalOcean with the given parameters,
    /// and the given key (property name for the response).
    fn send_request<T>(&mut self, req_params: DoRequest, key: &str) -> DoResult<T>
        where T: ::serde::Deserialize
    {
        use hyper::header::{Authorization, Bearer, ContentType};

        let DoRequest {
            relative_url,
            method,
            body
        } = req_params;

        let url = API_ROOT.join(&relative_url).unwrap();

        let auth_header = Authorization(Bearer { token: self.token.to_owned() });

        // XXX: this may be inefficient.
        let body: Vec<u8> = match body {
            Some(b) => b.into(),
            None => Vec::new(),
        };

        debug!("Sending body:\n{}", String::from_utf8_lossy(&body));

        let req_builder = self.http_client
                              .request(method, url)
                              .body(&body[..])
                              .header(auth_header)
                              .header(ContentType::json());

        let resp = try!(req_builder.send());

        let json_value = try!(serde_json::from_reader(resp));

        from_obj(json_value, key)
    }
}

// XXX: these .clone() calls are inefficient, but required to appease the
// borrowck's wrath.

/// Get a result from a JSON object and a property name.
fn from_obj<T>(value: serde_json::Value, key: &str) -> DoResult<T>
    where T: serde::Deserialize
{
    match value.find(key) {
        Some(v) => serde_json::from_value(v.clone()).map_err(Into::into),
        None => {
            match serde_json::from_value::<ApiError>(value.clone()) {
                Ok(api_error) => Err(DoError::Api(api_error)),
                Err(e) => Err(DoError::Json(e)),
            }
        }
    }
}
