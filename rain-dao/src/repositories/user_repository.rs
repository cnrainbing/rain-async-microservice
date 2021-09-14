pub struct UserRepository {}

impl UserRepository {
    pub async fn api_version() -> &'static str {
        "1.0"
    }
}