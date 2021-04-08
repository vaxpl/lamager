use rocket::FromForm;
use sha2::{Digest, Sha256};

#[derive(Clone, Debug, FromForm)]
pub struct NewPassword {
    pub old_password: String,
    pub new_password: String,
    pub new_password_confirm: String,
}

impl NewPassword {
    /// Returns base64 password digest with SHA265 algo.
    pub fn password_sha256(&self) -> String {
        let hash = Sha256::digest(self.new_password.as_bytes());
        base64::encode(hash)
    }
}
