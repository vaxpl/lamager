use crate::models::{NewPassword, NewUser, PasswordDigest, Person};
use ldap3::result::{LdapError, Result};
use ldap3::{LdapConn, Mod, Scope, SearchEntry};
use maplit::hashset;
use rocket::config::Value;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::State;
use std::collections::BTreeMap;

const DEFAULT_URI: &str = "ldap://127.0.0.1:10389";
const DEFAULT_BASE_DN: &str = "dc=example,dc=com";
const DEFAULT_ADMIN_DN: &str = "uid=admin,dc=example,dc=com";
const DEFAULT_ADMIN_PWD: &str = "password of admin";

/// The accessor for LDAP.
pub struct LdapAccessor {
    pub cfg: LdapConfig,
    pub con: LdapConn,
}

impl LdapAccessor {
    /// Construct a new ldap accessor.
    pub fn new(cfg: &LdapConfig) -> Result<Self> {
        Ok(Self {
            cfg: Clone::clone(cfg),
            con: LdapConn::new(&cfg.uri)?,
        })
    }

    /// Returns the entry under `base_dn` and the `uid` or `mail` match to `username`.
    pub fn entry_of_username<N>(&mut self, username: N) -> Result<SearchEntry>
    where
        N: AsRef<str>,
    {
        let filter = format!(
            "(&(objectClass=inetOrgPerson)(|(mail={})(uid={})))",
            username.as_ref(),
            username.as_ref()
        );
        let (rs, _res) = self
            .con
            .search(
                &self.cfg.base_dn,
                Scope::Subtree,
                &filter,
                vec!["uid", "cn", "sn", "mail", "photo", "createTimestamp"],
            )?
            .success()?;
        rs.into_iter()
            .next()
            .map(SearchEntry::construct)
            .ok_or(LdapError::EndOfStream)
    }

    pub fn new_user(&mut self, user: &NewUser) -> Result<()> {
        self.con
            .simple_bind(&self.cfg.admin_dn, &self.cfg.admin_pwd)?
            .success()?;
        let dn = format!("uid={},{}", user.uid, self.cfg.base_dn);
        let ssha256_pwd = format!("{{SSHA256}}{}", user.ssha256());
        self.con
            .add(
                &dn,
                vec![
                    ("objectClass", hashset! { "inetOrgPerson", "person" }),
                    ("uid", hashset! { user.uid.as_str() }),
                    ("cn", hashset! { user.cn.as_str() }),
                    ("sn", hashset! { user.cn.as_str() }),
                    ("mail", hashset! { user.mail.as_str() }),
                    ("userPassword", hashset! { ssha256_pwd.as_str() }),
                ],
            )?
            .success()?;
        Ok(())
    }

    /// Update user password to specfied with `user_dn`.
    pub fn update_password<D>(&mut self, user_dn: D, new_password: &NewPassword) -> Result<()>
    where
        D: AsRef<str>,
    {
        self.con
            .simple_bind(user_dn.as_ref(), new_password.old_password.as_ref())?
            .success()?;
        let ssha256_pwd = format!("{{SSHA256}}{}", new_password.ssha256());
        let mod_options = vec![Mod::Replace(
            "userPassword",
            hashset! { ssha256_pwd.as_str() },
        )];
        self.con.modify(user_dn.as_ref(), mod_options)?.success()?;
        Ok(())
    }

    /// Update person attributes to specfied with `user_dn`.
    pub fn update_person<D>(&mut self, user_dn: D, person: &Person) -> Result<()>
    where
        D: AsRef<str>,
    {
        self.con
            .simple_bind(&self.cfg.admin_dn, &self.cfg.admin_pwd)?
            .success()?;
        let mod_options = vec![
            Mod::Replace("cn", hashset! { person.cn.as_str() }),
            Mod::Replace("mail", hashset! { person.mail.as_str() }),
            // Mod::Replace("location", hashset! { person.location.as_str() }),
        ];
        self.con.modify(user_dn.as_ref(), mod_options)?.success()?;
        Ok(())
    }

    /// Update photo with `bytes` to specfied with `user_dn`.
    pub fn update_photo<D>(&mut self, user_dn: D, bytes: &[u8]) -> Result<()>
    where
        D: AsRef<str>,
    {
        self.con
            .simple_bind(&self.cfg.admin_dn, &self.cfg.admin_pwd)?
            .success()?;
        let photo_content = unsafe { std::str::from_utf8_unchecked(bytes) };
        let mod_options = vec![Mod::Replace("photo", hashset! { photo_content })];
        self.con.modify(user_dn.as_ref(), mod_options)?.success()?;
        Ok(())
    }

    /// Returns true if the the password matched.
    pub fn verify_password<D, P>(&mut self, user_dn: D, user_pwd: P) -> bool
    where
        D: AsRef<str>,
        P: AsRef<str>,
    {
        self.con
            .simple_bind(user_dn.as_ref(), user_pwd.as_ref())
            .map_or(false, |x| x.success().is_ok())
    }
}

impl Drop for LdapAccessor {
    fn drop(&mut self) {
        self.con.unbind().unwrap();
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for LdapAccessor {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let cfg = request.guard::<State<LdapConfig>>()?;
        match LdapAccessor::new(&cfg) {
            Ok(v) => Outcome::Success(v),
            Err(_) => Outcome::Forward(()),
        }
    }
}

/// Returns the string specified by name in the table.
fn table_get_string(table: &BTreeMap<String, Value>, name: &str, def_val: &str) -> String {
    table
        .get(name)
        .map_or(def_val, |x| x.as_str().unwrap_or(def_val))
        .to_string()
}

#[derive(Clone, Debug, Default)]
pub struct LdapConfig {
    pub uri: String,
    pub base_dn: String,
    pub admin_dn: String,
    pub admin_pwd: String,
}

impl From<&BTreeMap<String, Value>> for LdapConfig {
    fn from(table: &BTreeMap<String, Value>) -> Self {
        Self {
            uri: table_get_string(table, "uri", DEFAULT_URI),
            base_dn: table_get_string(table, "base_dn", DEFAULT_BASE_DN),
            admin_dn: table_get_string(table, "admin_dn", DEFAULT_ADMIN_DN),
            admin_pwd: table_get_string(table, "admin_pwd", DEFAULT_ADMIN_PWD),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_mail() {
        let mut la = LdapAccessor::new("ldap://10.0.6.238:10389").unwrap();
        let r = la
            .entry_of_username("ou=people,dc=vaxpl,dc=com", "varphone@qq.com")
            .unwrap();
        println!("r={:#?}", r);
        assert_eq!(la.verify_password(r.dn, "Var,,100200"), true);
    }
}
