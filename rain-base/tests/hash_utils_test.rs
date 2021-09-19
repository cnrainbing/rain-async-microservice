
use rain_base::hash::HashUtils;

///测试密码 编码和解码
#[test]
fn md5_str() {
    let s: String = HashUtils::md5_str("123456");
    println!("md5_str -> {}", s);
    assert_eq!(
        HashUtils::md5_str("123456"),
        HashUtils::md5_str("123456")
    )
}