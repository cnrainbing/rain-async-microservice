use regex::Regex;


lazy_static::lazy_static! {
    /// 用户名称正则
    pub static ref USERNAME_REGEX: Regex = Regex::new(r"^[a-zA-Z]+[a-zA-Z_0-9]{4,19}$").unwrap();
    /// 电子邮件正则
    pub static ref EMAIL_REGEX: Regex = Regex::new(r"/^([A-Za-z0-9_\-\.])+@([A-Za-z0-9_\-\.])+\.([A-Za-z]{2,5})$/").unwrap();
}