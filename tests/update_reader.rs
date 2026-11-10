mod common;

use book_manager::{
    AsyncUnitOfWork, Reader, UpdateReaderProfileError, async_update_reader,
    service::login::str_hash,
};

#[tokio::test]
async fn test_update_reader() {
    let (pool, mut uow) = common::setup().await;
    let account = format!("test_update_reader_{}", common::unique_suffix());
    let old_pwd = "old_password";
    let new_pwd = "new_password";

    {
        let mut repos = uow.repositories().await.expect("获取仓库失败");
        repos
            .reader()
            .add(Reader {
                account: account.clone(),
                name: "旧名字".to_string(),
                hash_pwd: str_hash(old_pwd),
            })
            .await
            .expect("新增读者失败");
    }

    let result = async_update_reader(&mut uow, &account, Some("新名字"), None, None).await;
    assert!(matches!(result, Ok(())), "更新读者姓名失败");

    let reader = {
        let mut repos = uow.repositories().await.expect("获取仓库失败");
        repos
            .reader()
            .find_reader_by_account(&account)
            .await
            .expect("查询读者失败")
    };
    assert_eq!(reader.name, "新名字");
    assert_eq!(reader.hash_pwd, str_hash(old_pwd));

    let result = async_update_reader(
        &mut uow,
        &account,
        None,
        Some("wrong_password"),
        Some(new_pwd),
    )
    .await;
    assert!(
        matches!(result, Err(UpdateReaderProfileError::PasswordError)),
        "当前密码错误时应返回 PasswordError"
    );

    let result = async_update_reader(&mut uow, &account, None, Some(old_pwd), Some(new_pwd)).await;
    assert!(matches!(result, Ok(())), "修改密码失败");

    let reader = {
        let mut repos = uow.repositories().await.expect("获取仓库失败");
        repos
            .reader()
            .find_reader_by_account(&account)
            .await
            .expect("查询读者失败")
    };
    assert_eq!(reader.hash_pwd, str_hash(new_pwd), "密码摘要应已更新");

    let result = async_update_reader(&mut uow, &account, Some("新名字"), None, None).await;
    assert!(
        matches!(result, Err(UpdateReaderProfileError::NoChanges)),
        "无改动时应返回 NoChanges"
    );

    let missing_account = format!("test_missing_{}", common::unique_suffix());
    let result = async_update_reader(&mut uow, &missing_account, Some("名字"), None, None).await;
    assert!(
        matches!(result, Err(UpdateReaderProfileError::NonExistedAccount)),
        "不存在的读者应返回 NonExistedAccount"
    );

    common::delete_reader(&pool, &account).await;
}
