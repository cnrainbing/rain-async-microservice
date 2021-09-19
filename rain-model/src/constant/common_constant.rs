use regex::Regex;

lazy_static::lazy_static! {
    /// 用户相关正则
    pub static ref EMAIL_REGEX: Regex = Regex::new(r"(@)").unwrap();
    pub static ref USERNAME_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9_-]{4,16}$").unwrap();
}