pub struct Client {
    token: String,
}

impl Client {
    pub fn with_token<T: Into<String>>(tok: T) -> Client {
        Client { token: tok.into() }
    }
}
