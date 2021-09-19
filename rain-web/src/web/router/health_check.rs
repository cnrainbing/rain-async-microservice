use actix_web::{HttpResponse, Responder};

/// 健康检查
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(r#"{"status": "up"}"#)
}
