use response;
use request::RequestBuilder;
use request::DoRequest;

impl<'t> DoRequest<response::Action> for ActionRequest<'t> {}
