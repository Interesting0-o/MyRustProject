mod cli;

use std::process;

use book_manager::{SqliteBaseNewError, SqliteUnitOfWork, UnitOfWork, new_sqlite_unit_of_work};

use crate::cli::{
    librarian::librarian_contrl,
    login::{LoginResult, login},
    reader::reader_contrl,
    sign_up::sign_up,
};

fn main() {
    //连接初始化：默认使用 sqlite 仓储，数据落盘到 resource/repo.db
    let mut uow = sqlite_unit_of_work();

    //主循环
    loop {
        let login_result = login(&uow);
        match login_result {
            LoginResult::Quit => break,
            //注册完回到登录界面
            LoginResult::SignUp => {
                sign_up(&uow);
            }
            LoginResult::ReaderUser(cur_reader) => {
                reader_contrl(&mut uow, &cur_reader);
            }
            LoginResult::LibrarianUser(_) => {
                let repos = uow.repositories();
                librarian_contrl(&*repos.book());
            }
        }
    }
}

/// 构造 sqlite 工作单元：打开 `resource/repo.db` 并初始化所有表。
fn sqlite_unit_of_work() -> SqliteUnitOfWork {
    const DB_PATH: &str = "resource/repo.db";
    match new_sqlite_unit_of_work(DB_PATH) {
        Ok(uow) => uow,
        Err(SqliteBaseNewError::PathIsEmpty) => {
            eprintln!("数据库路径为空：{DB_PATH}");
            process::exit(1);
        }
        Err(SqliteBaseNewError::ConnectError(e)) => {
            eprintln!("连接数据库失败：{DB_PATH}：{e}");
            process::exit(1);
        }
    }
}
