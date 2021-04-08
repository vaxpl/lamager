use rand::Rng;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::State;
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::{Arc, RwLock};

#[derive(Clone, Debug, Default)]
pub struct Session {
    pub ssid: String,
    pub dn: String,
    pub uid: String,
}

impl Session {
    /// Make a randomized session id.
    fn make_ssid() -> String {
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                abcdefghijklmnopqrstuvwxyz\
                                0123456789";
        const KEY_LEN: usize = 8;
        let mut rng = rand::thread_rng();

        (0..KEY_LEN)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect()
    }

    /// Construct a new session with `dn` and `uid`.
    pub fn new(dn: String, uid: String) -> Self {
        Self {
            ssid: Self::make_ssid(),
            dn,
            uid,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum SessionError {
    NoCookie,
    NoManager,
    NotFound,
}

// impl From<()> for SessionError {
//     fn from(_: ()) -> Self {
//         SessionError::NoManager
//     }
// }

#[derive(Clone, Debug)]
pub struct SessionRef(pub Arc<Session>);

impl SessionRef {
    /// Construct a new session with `dn` and `uid`.
    pub fn new(dn: String, uid: String) -> Self {
        Self(Arc::new(Session::new(dn, uid)))
    }
}

impl Deref for SessionRef {
    type Target = Arc<Session>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for SessionRef {
    type Error = SessionError;

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let manager = request
            .guard::<State<SessionManager>>()
            .map_failure(|_| (Status::BadRequest, SessionError::NoManager))?;
        let session = request
            .cookies()
            .get_private("ssid")
            .ok_or(SessionError::NoCookie)
            .and_then(|cookie| manager.get(cookie.value()).ok_or(SessionError::NotFound));
        match session {
            Ok(s) => Outcome::Success(s),
            Err(_) => Outcome::Forward(()),
        }
    }
}

pub struct SessionManager {
    sessions: RwLock<HashMap<String, SessionRef>>,
}

impl SessionManager {
    /// Construct a new session manager.
    pub fn new() -> Self {
        Self {
            sessions: RwLock::new(HashMap::new()),
        }
    }

    /// Add a session to table.
    pub fn add(&self, session: SessionRef) {
        if let Ok(mut x) = self.sessions.write() {
            x.insert(Clone::clone(&session.ssid), session);
        }
    }

    /// Returns the reference of the session in the table specified by `k`.
    pub fn get(&self, k: &str) -> Option<SessionRef> {
        self.sessions
            .read()
            .map(|x| x.get(k).map(|x| Clone::clone(x)))
            .ok()
            .flatten()
    }

    /// Remove the session in the table specified by `k`.
    pub fn remove(&self, k: &str) {
        if let Ok(mut x) = self.sessions.write() {
            x.remove(k);
        }
    }
}
