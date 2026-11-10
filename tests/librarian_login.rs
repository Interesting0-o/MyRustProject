mod common;

use book_manager::{LoginError, async_librarian_login, async_librarian_sign_up};

#[tokio::test]
async fn test_librarian_login() {
    let (pool, mut uow) = common::setup().await;
    let account = format!("test_lib_{}", common::unique_suffix());
    let name = "测试管理员";
    let pwd = "123456";

    assert!(
        matches!(
            async_librarian_sign_up(&mut uow, name, &account, pwd).await,
            Ok(())
        ),
        "注册管理员失败"
    );

    let librarian = async_librarian_login(&mut uow, &account, pwd)
        .await
        .unwrap_or_else(|_| panic!("管理员登录失败"));
    assert_eq!(librarian.account, account);
    assert_eq!(librarian.name, name);

    let err = async_librarian_login(&mut uow, &account, "wrong_password")
        .await
        .expect_err("错误密码不应登录成功");
    assert!(matches!(err, LoginError::PasswordError));

    let missing_account = format!("test_missing_{}", common::unique_suffix());
    let err = async_librarian_login(&mut uow, &missing_account, pwd)
        .await
        .expect_err("不存在的账号不应登录成功");
    assert!(matches!(err, LoginError::AccountNotFound));

    common::delete_librarian(&pool, &account).await;
}
