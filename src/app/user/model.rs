use crate::schema::users;
use crate::utils::hasher;
use crate::{error::AppError, utils::token};
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use serde::Deserialize;
use uuid::Uuid;
#[derive(Insertable, Queryable)]
#[table_name = "users"]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub password: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Debug, Deserialize)]
#[table_name = "users"]
pub struct SignupUser<'a> {
    pub email: &'a str,
    pub username: &'a str,
    pub password: &'a str,
}

type Token = String;

impl User {
    pub fn signup<'a>(
        conn: &PgConnection,
        email: &'a str,
        username: &'a str,
        naive_password: &'a str,
    ) -> Result<(User, Token), AppError> {
        use diesel::prelude::*;

        let start_hash = Utc::now().timestamp_millis();
        let hashed_passwd = hasher::hash_password(naive_password)?;
        let record = SignupUser {
            email,
            username,
            password: &hashed_passwd,
        };

        let start_insert = Utc::now().timestamp_millis();
        let user = diesel::insert_into(users::table)
            .values(&record)
            .get_result::<User>(conn)?;

        let start_generate_token = Utc::now().timestamp_millis();
        let token = user.generate_token()?;

        Ok((user, token))
    }

    pub fn find_by_username(conn: &PgConnection, username: &str) -> Result<Self, AppError> {
        let user = users::table
            .filter(users::username.eq(username))
            .first::<User>(conn)?;
        Ok(user)
    }
}

impl User {
    pub fn generate_token(&self) -> Result<String, AppError> {
        let now = Utc::now().timestamp();
        let token = token::generate(self.id, now)?;
        Ok(token)
    }
}
