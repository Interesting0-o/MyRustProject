mod common;

use book_manager::{
    AsyncUnitOfWork, SignUpRes, async_librarian_sign_up, async_reader_sign_up,
    service::login::str_hash,
};

#[tokio::test]
async fn test_reader_sign_up() {
    let (pool, mut uow) = common::setup().await;
    let account = format!("test_reader_{}", common::unique_suffix());
    let name = "测试读者";
    let pwd = "123456";

    assert!(
        matches!(
            async_reader_sign_up(&mut uow, name, &account, pwd).await,
            Ok(())
        ),
        "注册读者失败"
    );

    let reader = {
        let mut repos = uow.repositories().await.expect("获取仓库失败");
        repos
            .reader()
            .find_reader_by_account(&account)
            .await
            .expect("未找到刚注册的读者")
    };
    assert_eq!(reader.name, name);
    assert_eq!(reader.hash_pwd, str_hash(pwd));

    let result = async_reader_sign_up(&mut uow, "另一个名字", &account, "654321").await;
    assert!(
        matches!(result, Err(SignUpRes::AccountAlreadyExists)),
        "重复账号应注册失败"
    );

    common::delete_reader(&pool, &account).await;
}

#[tokio::test]
async fn test_librarian_sign_up() {
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

    let librarian = {
        let mut repos = uow.repositories().await.expect("获取仓库失败");
        repos
            .librarian()
            .find_librarian_by_account(&account)
            .await
            .expect("未找到刚注册的管理员")
    };
    assert_eq!(librarian.name, name);
    assert_eq!(librarian.hash_pwd, str_hash(pwd));

    let result = async_librarian_sign_up(&mut uow, "另一个名字", &account, "654321").await;
    assert!(
        matches!(result, Err(SignUpRes::AccountAlreadyExists)),
        "重复账号应注册失败"
    );

    common::delete_librarian(&pool, &account).await;
}
