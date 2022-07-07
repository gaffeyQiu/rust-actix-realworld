use actix_web::{web, HttpResponse};

use crate::{middleware::state::AppState, utils::api::ApiResponse};

use super::{model::User, request, response::UserResponse};

pub async fn signup(state: web::Data<AppState>, form: web::Json<request::Signup>) -> ApiResponse {
    let conn = state.get_conn()?;
    let start_time = chrono::Utc::now().timestamp_millis();
    let (user, token) = User::signup(
        &conn,
        &form.user.email,
        &form.user.username,
        &form.user.password,
    )?;
    let res = UserResponse::from((user, token));
    Ok(HttpResponse::Ok().json(res))
}
