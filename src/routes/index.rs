use crate::models::SessionRef;
use rocket::response::Redirect;
use rocket::Route;

#[get("/")]
pub(crate) fn index(_session: SessionRef) -> Redirect {
    Redirect::to(uri!(crate::routes::profile::profile))
}

#[get("/", rank = 2)]
pub(crate) fn index_without_session() -> Redirect {
    Redirect::to(uri!(crate::routes::login::login_page))
}

pub fn routes() -> Vec<Route> {
    routes![index, index_without_session]
}
