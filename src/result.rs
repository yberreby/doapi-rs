use std::io;

quick_error! {
    #[derive(Debug)]
    pub enum DoError {
        Api(e: ApiError) {
            from()
        }
        Io(e: io::Error) {
            from()
        }
        Json(e: ::serde_json::Error) {
            from()
        }
        Http(e: ::hyper::Error) {
            from()
        }
    }
}

pub type DoResult<T> = ::std::result::Result<T, DoError>;

#[derive(Deserialize, Debug)]
pub struct ApiError {
    id: String,
    message: String,
}
