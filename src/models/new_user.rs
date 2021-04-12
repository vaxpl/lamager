use super::PasswordDigest;
use rocket::FromForm;

#[derive(Clone, Debug, FromForm)]
pub struct NewUser {
    pub uid: String,
    pub cn: String,
    pub mail: String,
    pub password: String,
    pub password_confirm: String,
}

impl PasswordDigest for NewUser {
    fn password(&self) -> &str {
        &self.password
    }
}
