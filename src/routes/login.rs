use crate::ldap::LdapAccessor;
use crate::models::{Login, SessionManager, SessionRef, User};
use rocket::http::{Cookie, Cookies};
use rocket::request::{FlashMessage, Form};
use rocket::response::{Flash, Redirect};
use rocket::{Route, State};
use rocket_contrib::templates::Template;
use std::collections::HashMap;

#[post("/login", data = "<login>")]
pub(crate) fn login(
    login: Form<Login>,
    session_manager: State<SessionManager>,
    mut cookies: Cookies,
    mut ldap: LdapAccessor,
) -> Result<Redirect, Flash<Redirect>> {
    if let Ok(entry) = ldap.entry_of_username(&login.username) {
        let dn = &entry.dn;
        let uid = &entry.attrs["uid"][0];
        if ldap.verify_password(&dn, &login.password) {
            let session = SessionRef::new(Clone::clone(dn), Clone::clone(uid));
            cookies.add_private(Cookie::new("ssid", session.ssid.to_string()));
            session_manager.add(session);
            Ok(Redirect::to(uri!(crate::routes::index::index)))
        } else {
            Err(Flash::error(
                Redirect::to(uri!(login_page)),
                "用户名或密码有误，请重新输入！",
            ))
        }
    } else {
        Err(Flash::error(
            Redirect::to(uri!(login_page)),
            "用户账号或邮箱不存在，请重新输入！",
        ))
    }
}

#[get("/login")]
pub(crate) fn login_user(_user: User) -> Redirect {
    Redirect::to(uri!(crate::routes::index::index))
}

#[get("/login", rank = 2)]
pub(crate) fn login_page(flash: Option<FlashMessage>) -> Template {
    let mut context = HashMap::new();
    if let Some(ref msg) = flash {
        context.insert("flash", msg.msg());
    }

    Template::render("login", &context)
}

pub fn routes() -> Vec<Route> {
    routes![login, login_user, login_page]
}
