use actix_web::HttpResponse;
use actix_web::HttpRequest;
use actix_web::middleware::session::*;
use actix_web::middleware::{Middleware, Response, Started};
use error::{Error, ResponseError, Result};
// use http::HeaderMap;
use actix_web::http::header::{AUTHORIZATION};

pub struct AuthInfoInner {
    uid: String,
    roles: Vec<String>,
}
pub enum AuthInfo {
    NoData(),
    Invalid(),
    Valid(AuthInfoInner),
}

pub struct AuthSession {
    auth: AuthInfo,

}
pub struct AuthSessionBackend {}

impl AuthInfo {
    pub fn load<S>(req: &mut HttpRequest<S>) -> AuthInfo {
        match req.headers().get(AUTHORIZATION) {
            None => AuthInfo::NoData(),
            Some(headerVal) => match headerVal.to_str() {
                Err(_) => AuthInfo::Invalid(),
                Ok(authStr) => {
                    debug!("Authrorization String: {}", authStr);
                    // TODO parse authStr ...
                    AuthInfo::Valid(AuthInfoInner {
                        uid: String::from("123uid-notimpl-emente-dyet"),
                        roles: vec![String::from("_test-role")],
                    })
                },
            }
        }
    }
}

impl SessionImpl for AuthSession {
    fn get(&self, key: &str) -> Option<&str>;

    fn set(&mut self, key: &str, value: String);

    fn remove(&mut self, key: &str);

    fn clear(&mut self);

    /// Write session to storage backend.
    fn write(&self, resp: HttpResponse) -> Result<Response> {
        // sure! lets write everything to our null backend ...
        Ok(Response::Done(resp))
        // and done!
    }
}
impl<S> SessionBackend<S> for AuthSessionBackend {
    type Session = AuthSession;
    type ReadFuture = FutureResult<AuthSession, Error>;

    fn from_request(&self, req: &mut HttpRequest<S>) -> Self::ReadFuture {
        let state = self.0.load(req);
        FutOk(CookieSession {
            changed: false,
            inner: Rc::clone(&self.0),
            state,
        })
    }
}

// pub trait SessionBackend<S>: Sized + 'static {
//     type Session: SessionImpl;
//     type ReadFuture: Future<Item = Self::Session, Error = Error>;
//
//     /// Parse the session from request and load data from a storage backend.
//     fn from_request(&self, request: &mut HttpRequest<S>) -> Self::ReadFuture;
// }
