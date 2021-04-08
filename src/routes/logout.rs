use crate::models::{SessionManager, SessionRef};
use rocket::http::{Cookie, Cookies};
use rocket::response::{Flash, Redirect};
use rocket::{Route, State};

#[get("/logout")]
pub(crate) fn logout(
    session: SessionRef,
    session_manager: State<SessionManager>,
    mut cookies: Cookies,
) -> Flash<Redirect> {
    cookies.remove_private(Cookie::named("ssid"));
    session_manager.remove(&session.ssid);
    Flash::success(
        Redirect::to(uri!(crate::routes::login::login_page)),
        "当前会话已注销，请重新登录！",
    )
}

#[get("/logout", rank = 2)]
pub(crate) fn logout_without_session(mut cookies: Cookies) -> Flash<Redirect> {
    cookies.remove_private(Cookie::named("ssid"));
    Flash::success(
        Redirect::to(uri!(crate::routes::login::login_page)),
        "请先登录再执行其他操作！",
    )
}

pub fn routes() -> Vec<Route> {
    routes![logout, logout_without_session]
}
