use rocket::FromForm;

#[derive(Debug, FromForm)]
pub struct Login {
    pub username: String,
    pub password: String,
}
