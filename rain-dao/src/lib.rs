use rain_base::hash::hash_utils::HashUtils;

pub async fn md5_str() -> String {
    HashUtils::md5_str("ddd")
}