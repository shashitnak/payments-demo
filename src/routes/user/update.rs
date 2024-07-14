use crate::db::{user, User};
use crate::routes::user_extractor::UserExtractor;
use crate::routes::Result;
use crate::AppData;
use actix_web::{post, web};
use serde::Deserialize;

#[derive(Deserialize)]
struct Request {
    name: Option<String>,
    email: Option<String>,
    username: Option<String>,
    password: Option<String>,
}

#[post("/updateUser")]
pub async fn update_user(
    user_extractor: UserExtractor,
    app_data: web::Data<AppData>,
    req: web::Json<Request>,
) -> Result<web::Json<User>> {
    let query = user::Update {
        id: user_extractor.user.id,
        name: req.name.as_deref(),
        email: req.email.as_deref(),
        username: req.username.as_deref(),
        password: req.password.as_deref(),
    };

    let user = app_data.db_conn.run_query(query).await?;
    Ok(web::Json(user))
}
