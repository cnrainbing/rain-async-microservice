use md5::Digest;

pub struct HashUtils {}

impl HashUtils {
    pub fn md5_str(raw_password: &str) -> String {
        if raw_password.is_empty() {
            panic!("raw_password is not empty");
        }
        let digest: Digest = md5::compute(raw_password);
        format!("{:x}", digest)
    }

    pub fn verify(password: &str, raw_password: &str) -> bool {
        if password.eq(raw_password) {
            return true;
        }
        let hashed: String = HashUtils::md5_str(raw_password);
        password.eq(&hashed)
    }

    pub fn md5_string(raw_password: String) -> String {
        if raw_password.is_empty() {
            panic!("raw_password is not empty");
        }
        let digest: Digest = md5::compute(raw_password);
        format!("{:x}", digest)
    }

    pub fn md5_bytes(raw_password: &[u8]) -> String {
        if raw_password.is_empty() {
            panic!("raw_password is not empty");
        }
        let digest: Digest = md5::compute(raw_password);
        format!("{:x}", digest)
    }
}

///测试模块
#[cfg(test)]
mod test {
    use crate::hash::hash_utils::HashUtils;

    ///测试密码 编码和解码
    #[test]
    fn md5_str() {
        let s: String = HashUtils::md5_str("123456");
        println!("{}", s);
        assert_eq!(
            HashUtils::md5_str("123456"),
            HashUtils::md5_str("123456")
        )
    }

    #[test]
    fn test_verify() {
        let password: &str = "12345";
        let raw_password: &str = "12345";

        assert!(HashUtils::verify(password, raw_password));

        let encode_password: String = HashUtils::md5_bytes(password.as_bytes());
        assert!(HashUtils::verify(&encode_password, password));
    }
}