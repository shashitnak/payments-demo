use crate::db::user;
use crate::routes::Result;
use crate::AppData;
use actix_web::{post, web};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Request {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct Response {
    token: String,
}

#[post("/signIn")]
pub async fn sign_in(
    app_data: web::Data<AppData>,
    req: web::Json<Request>,
) -> Result<web::Json<Response>> {
    let query = user::Check {
        username: &req.username,
        password: &req.password,
    };

    let id = app_data.db_conn.run_query(query).await?;

    let token = app_data.jwt.generate_token(id)?;
    Ok(web::Json(Response { token }))
}
