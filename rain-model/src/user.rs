use async_graphql::{SimpleObject,ComplexObject};
use serde::Deserialize;
use serde::Serialize;
use chrono::{DateTime, Utc, Local};

#[derive(SimpleObject, Deserialize, Serialize)]
#[graphql(complex)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub email: String,
    pub email_verified: bool,
    pub password: String,
    pub image: Option<String>,
    #[graphql(skip)]
    pub created_at: DateTime<Utc>,
    #[graphql(skip)]
    pub updated_at: DateTime<Utc>,
}

#[ComplexObject]
impl User {
    async fn created_at(&self) -> DateTime<Local> {
        self.created_at.with_timezone(&Local)
    }

    async fn updated_at(&self) -> DateTime<Local> {
        self.updated_at.with_timezone(&Local)
    }
}
