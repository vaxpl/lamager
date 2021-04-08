use rocket::outcome::IntoOutcome;
use rocket::request::{self, FromRequest, Request};

#[derive(Debug)]
pub struct User(pub usize);

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = !;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<User, !> {
        request
            .cookies()
            .get_private("user_id")
            .and_then(|cookie| cookie.value().parse().ok())
            .map(User)
            .or_forward(())
    }
}
