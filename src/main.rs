mod cli;

use std::process;

use book_manager::{
    SqliteBaseBookRepository, SqliteBaseBorrowRecordRepository, SqliteBaseLibrarianRepository,
    SqliteBaseNewError, SqliteBaseReaderRepository,
};

use crate::cli::{
    librarian::librarian_contrl,
    login::{LoginResult, login},
    reader::reader_contrl,
    sign_up::sign_up,
};

/// 默认使用的 sqlite 仓储组：供主循环与各控制器共享。
struct Repositories {
    book: SqliteBaseBookRepository,
    reader: SqliteBaseReaderRepository,
    librarian: SqliteBaseLibrarianRepository,
    borrow_record: SqliteBaseBorrowRecordRepository,
}

fn main() {
    //仓库初始化：默认使用 sqlite 仓储，数据落盘到 resource/repo.db
    let repos = sqlite_repositories();

    //主循环
    loop {
        match login(&repos.reader, &repos.librarian) {
            LoginResult::Quit => break,
            //注册完回到登录界面
            LoginResult::SignUp => {
                sign_up(&repos.reader, &repos.librarian);
            }
            LoginResult::ReaderUser(cur_reader) => {
                reader_contrl(&repos.book, &repos.borrow_record, &cur_reader);
            }
            LoginResult::LibrarianUser(_) => {
                librarian_contrl(&repos.book);
            }
        }
    }
}

/// sqlite 仓库生成器：四个仓储共享 `resource/repo.db`，进程退出后仍然保留。
fn sqlite_repositories() -> Repositories {
    const DB_PATH: &str = "resource/repo.db";
    Repositories {
        book: open_sqlite_repo("图书", DB_PATH, SqliteBaseBookRepository::new),
        reader: open_sqlite_repo("读者", DB_PATH, SqliteBaseReaderRepository::new),
        librarian: open_sqlite_repo("管理员", DB_PATH, SqliteBaseLibrarianRepository::new),
        borrow_record: open_sqlite_repo(
            "借阅记录",
            DB_PATH,
            SqliteBaseBorrowRecordRepository::new,
        ),
    }
}

/// 构造一个 sqlite 仓储；失败时打印原因并结束进程。
fn open_sqlite_repo<T>(
    name: &str,
    path: &str,
    open: impl FnOnce(&str) -> Result<T, SqliteBaseNewError>,
) -> T {
    match open(path) {
        Ok(repo) => repo,
        Err(SqliteBaseNewError::PathIsEmpty) => {
            eprintln!("{name}仓储路径为空：{path}");
            process::exit(1);
        }
        Err(SqliteBaseNewError::ConnectError(e)) => {
            eprintln!("连接{name}仓储失败：{path}：{e}");
            process::exit(1);
        }
    }
}

