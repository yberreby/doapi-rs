use std::io;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Io(e: io::Error) {
            from()
        }
        Json(e: ::serde_json::Error) {
            from()
        }
    }
}

pub type DoResult<T> = ::std::result::Result<T, Error>;
