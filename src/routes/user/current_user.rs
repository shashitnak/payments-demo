use crate::db::User;
use crate::routes::Result;
use crate::routes::UserExtractor;
use actix_web::{get, web};

#[get("/currentUser")]
pub async fn get_current_user(user_extractor: UserExtractor) -> Result<web::Json<User>> {
    Ok(web::Json(user_extractor.user))
}
