use actix_web::{HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct HealthCheck {
    pub status: String,
}

impl HealthCheck {
    pub fn new(status: String) -> HealthCheck{
        HealthCheck{status}
    }
}

/// 健康检查
pub async fn health_check() -> impl Responder {
    //HttpResponse::Ok().json(r#"{"status": "up"}"#)
    let health_check = HealthCheck::new(String::from("UP"));
    HttpResponse::Ok().json(health_check)
}
