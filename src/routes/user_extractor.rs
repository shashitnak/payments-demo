use crate::db::{user, User};
use crate::routes::Error;
use crate::routes::Error::NoLoggedInUser;
use crate::AppData;
use actix_web::dev::Payload;
use actix_web::http::header::HeaderValue;
use actix_web::{web, FromRequest, HttpRequest};
use std::future::Future;
use std::pin::Pin;

fn authorization_required(route: &str) -> bool {
    matches!(route, "/signUp" | "/signIn")
}

pub struct UserExtractor {
    pub(crate) user: User,
}

impl FromRequest for UserExtractor {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        if !authorization_required(req.path()) {
            let token = req
                .headers()
                .get("Authorization")
                .map(HeaderValue::to_str)
                .and_then(Result::ok)
                .and_then(|token| token.strip_prefix("Bearer "));

            if let Some(token) = token {
                let app_data: &web::Data<AppData> = req.app_data().unwrap();
                let app_data = app_data.clone().into_inner();
                if let Ok(claims) = app_data.jwt.validate_token(token) {
                    return Box::pin(async move {
                        let query = user::Select { id: claims.id };
                        let user = app_data.db_conn.run_query(query).await?;
                        Ok(UserExtractor { user })
                    });
                }
            }
        }

        Box::pin(async move { Err(NoLoggedInUser) })
    }
}
