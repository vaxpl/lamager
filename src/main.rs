#![feature(proc_macro_hygiene, decl_macro, never_type)]
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
use crate::ldap::LdapConfig;
use crate::models::SessionManager;
use rocket::fairing::AdHoc;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

mod ldap;
mod models;
mod routes;

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .attach(Template::fairing())
        .attach(AdHoc::on_attach("Ldap Config", |rocket| {
            let ldap = LdapConfig::from(rocket.config().get_table("ldap").unwrap());
            Ok(rocket.manage(ldap))
        }))
        .manage(SessionManager::new())
        .mount("/", routes::index::routes())
        .mount("/index", routes::index::routes())
        .mount("/", routes::login::routes())
        .mount("/", routes::logout::routes())
        .mount("/", routes::profile::routes())
        .mount("/", routes::recover::routes())
        .mount("/", routes::register::routes())
        .mount("/assets", StaticFiles::from("assets"))
        .mount("/favicon.ico", StaticFiles::from("assets/favicon.ico"))
}

fn main() {
    rocket().launch();
}
