use super::PasswordDigest;
use rocket::FromForm;

#[derive(Clone, Debug, FromForm)]
pub struct NewPassword {
    pub old_password: String,
    pub new_password: String,
    pub new_password_confirm: String,
}

impl PasswordDigest for NewPassword {
    fn password(&self) -> &str {
        &self.new_password
    }
}
