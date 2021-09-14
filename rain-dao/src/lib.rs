pub mod repositories;
use rain_base::hash::HashUtils;


pub fn md5_str() -> String {
    HashUtils::md5_str("ddd")
}