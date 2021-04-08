use rocket::FromForm;
use sha2::{Digest, Sha256};

#[derive(Clone, Debug, FromForm)]
pub struct NewUser {
    pub uid: String,
    pub cn: String,
    pub mail: String,
    pub password: String,
    pub password_confirm: String,
}

impl NewUser {
    /// Returns base64 password digest with SHA265 algo.
    pub fn password_sha256(&self) -> String {
        let hash = Sha256::digest(self.password.as_bytes());
        base64::encode(hash)
    }
}
