use crate::ldap::LdapAccessor;
use crate::models::NewUser;
use rocket::request::{FlashMessage, Form};
use rocket::response::{Flash, Redirect};
use rocket::Route;
use rocket_contrib::templates::Template;
use std::collections::HashMap;

#[post("/register", data = "<user>")]
pub(crate) fn register(user: Form<NewUser>, mut ldap: LdapAccessor) -> Flash<Redirect> {
    let user = user.into_inner();
    match ldap.new_user(&user) {
        Ok(_) => Flash::success(
            Redirect::to(uri!(crate::routes::index::index)),
            "注册账号成功，请登录核实或执行其他操作！",
        ),
        Err(_) => Flash::error(
            Redirect::to(uri!(register_empty)),
            "注册账号失败，请仔细检查各项内容后再次尝试！",
        ),
    }
}

#[get("/register")]
pub(crate) fn register_empty(flash: Option<FlashMessage>) -> Template {
    println!("flash={:?}", flash);
    let mut context = HashMap::new();
    if let Some(ref msg) = flash {
        context.insert("flash", msg.msg());
    }
    Template::render("register", &context)
}

pub fn routes() -> Vec<Route> {
    routes![register, register_empty]
}
