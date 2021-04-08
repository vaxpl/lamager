use rocket::Route;
use rocket_contrib::templates::Template;
use std::collections::HashMap;

#[get("/recover")]
pub(crate) fn recover() -> Template {
    let context: HashMap<String, String> = HashMap::new();
    Template::render("recover", &context)
}

pub fn routes() -> Vec<Route> {
    routes![recover]
}
