use actix_web::{get, web::Json};

use crate::models::User;

#[get("profile")]
pub async fn profile(user: User) -> Json<User> {
    Json(user)
}
