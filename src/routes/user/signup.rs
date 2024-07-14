use crate::db::user;
use crate::routes::Result;
use crate::AppData;
use actix_web::{post, web};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Request {
    name: String,
    email: String,
    username: String,
    password: String,
}

#[derive(Serialize)]
struct Response {
    token: String,
}

#[post("/signUp")]
pub async fn sign_up(
    app_data: web::Data<AppData>,
    req: web::Json<Request>,
) -> Result<web::Json<Response>> {
    let user_query = user::Insert {
        name: &req.name,
        email: &req.email,
        username: &req.username,
        password: &req.password,
    };
    let id = app_data.db_conn.run_query(user_query).await?;

    let token = app_data.jwt.generate_token(id)?;
    Ok(web::Json(Response { token }))
}
