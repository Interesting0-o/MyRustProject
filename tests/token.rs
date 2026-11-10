use book_manager::service::token::{check_token, create_token};

#[test]
fn test_create_and_check_token() {
    let token = create_token("r001", "reader", "test_secret", 3600).expect("签发 token 失败");

    let claims = check_token(&token, "test_secret").expect("校验 token 失败");
    assert_eq!(claims.sub, "r001");
    assert_eq!(claims.role, "reader");
    assert_eq!(claims.exp - claims.iat, 3600);
}

#[test]
fn test_check_token_with_wrong_secret() {
    let token = create_token("r001", "reader", "test_secret", 3600).expect("签发 token 失败");

    assert!(
        check_token(&token, "wrong_secret").is_err(),
        "错误密钥不应校验通过"
    );
}

#[test]
fn test_check_token_with_tampered_token() {
    let token = create_token("r001", "reader", "test_secret", 3600).expect("签发 token 失败");
    let tampered = format!("{token}x");

    assert!(
        check_token(&tampered, "test_secret").is_err(),
        "被篡改的 token 不应校验通过"
    );
}
