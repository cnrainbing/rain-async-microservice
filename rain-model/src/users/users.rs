use async_graphql::{SimpleObject, ComplexObject, InputObject};
use serde::Deserialize;
use serde::Serialize;
use chrono::{DateTime, Utc, Local};
use validator::Validate;
use crate::constant::USERNAME_REGEX;
use sqlx::FromRow;

#[derive(SimpleObject,FromRow, Deserialize, Serialize)]
#[graphql(complex)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub nickname: String,
    pub email: String,
    pub email_verified: i8,
    pub password_hash: String,
    pub active: i8,
    #[graphql(skip)]
    pub created_at: Option<DateTime<Utc>>,
    #[graphql(skip)]
    pub updated_at: Option<DateTime<Utc>>,
}

#[ComplexObject]
impl User {
    async fn created_at(&self) -> DateTime<Local> {
        self.created_at.unwrap().with_timezone(&Local)
    }

    async fn updated_at(&self) -> DateTime<Local> {
        self.updated_at.unwrap().with_timezone(&Local)
    }
}

/// 用户注册
#[derive(Serialize, Deserialize, InputObject, Validate)]
pub struct CreateUser {
    #[validate(regex(path = "USERNAME_REGEX", message = "用户名不符合要求"))]
    pub username: String,
    #[validate(email(message = "邮箱不符合"))]
    pub email: String,
    #[validate(length(min = 6, message = "密码不符合"))]
    pub password: String,
    #[validate(length(min = 3, message = "昵称不符合"))]
    pub nickname: String,
}

/// 用户注册
#[derive(Serialize, Deserialize, InputObject, Validate)]
pub struct LoginUser {
    #[validate(length(min = 1, message = "登录名称不符合要求"))]
    pub login: String,
    #[validate(length(min = 6, message = "密码不符合"))]
    pub password: String,
}

/// 用户登录token结构体
#[derive(SimpleObject)]
pub struct UsersToken {
    pub access_token: String,
    pub refresh_token: String,
    pub expires: i64,
}