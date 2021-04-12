use sha2::{Digest, Sha256};

/// The default length of the salt.
const SSHA256_SLAT_LEN: usize = 8;

/// A trait to generate password digests.
pub trait PasswordDigest {
    /// Returns the reference to password plain text.
    fn password(&self) -> &str;

    /// Returns base64 password digest with SHA256 algo.
    fn sha256(&self) -> String {
        let hash = Sha256::digest(self.password().as_bytes());
        base64::encode(hash)
    }

    /// Returns base64 password digest with SHA256+SALT algo.
    fn ssha256(&self) -> String {
        let salt = make_salt(SSHA256_SLAT_LEN);
        let mut buffer = Vec::new();
        buffer.extend(self.password().as_bytes());
        buffer.extend(&salt);
        let mut hash = Vec::new();
        hash.extend(Sha256::digest(&buffer));
        hash.extend(salt);
        base64::encode(hash)
    }
}

/// Returns a randomize salt.
fn make_salt(len: usize) -> Vec<u8> {
    (0..len).map(|_| rand::random::<u8>()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_digest() {
        // Implements trait for str
        impl PasswordDigest for &str {
            fn password(&self) -> &str {
                self
            }
        }
        // Test empty string
        assert_eq!("".sha256(), "47DEQpj8HBSa+/TImW+5JCeuQeRkm5NMpJWZG3hSuFU=");
        assert_ne!("".ssha256(), "47DEQpj8HBSa+/TImW+5JCeuQeRkm5NMpJWZG3hSuFU=");
        // Test normal string
        assert_eq!(
            "password".sha256(),
            "XohImNooBHFR0OVvjcYpJ3NgPQ1qq73WKhHvch0VQtg="
        );
        assert_ne!(
            "password".ssha256(),
            "XohImNooBHFR0OVvjcYpJ3NgPQ1qq73WKhHvch0VQtg="
        );
        // Test supper long string
        assert_eq!(
            "the is a long long long long long long long long long password".sha256(),
            "8S+xwsrFP7DnznPchhI+KTKtCeqloabKNTkcch+lIZA="
        );
        assert_ne!(
            "the is a long long long long long long long long long password".ssha256(),
            "8S+xwsrFP7DnznPchhI+KTKtCeqloabKNTkcch+lIZA="
        );
    }
}
