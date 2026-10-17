mod cli;

use std::process;

use book_manager::{
    SqliteBaseBookRepository, SqliteBaseBorrowRecordRepository, SqliteBaseLibrarianRepository,
    SqliteBaseNewError, SqliteBaseReaderRepository,
    infrastructure::sqlite_base::{
        sqlite_base_book_repo::new_book_table,
        sqlite_base_borrow_record_repo::new_borrow_record_table,
        sqlite_base_librarian_repo::new_librarian_table, sqlite_base_reader_repo::new_reader_table,
    },
};
use rusqlite::Connection;

use crate::cli::{
    librarian::librarian_contrl,
    login::{LoginResult, login},
    reader::reader_contrl,
    sign_up::sign_up,
};

/// 默认使用的 sqlite 连接组：供主循环与各控制器共享。
struct Connections {
    book_conn: Connection,
    borrow_record_conn: Connection,
    reader_conn: Connection,
    librarian_conn: Connection,
}

fn main() {
    //连接初始化：默认使用 sqlite 仓储，数据落盘到 resource/repo.db
    let conns = sqlite_connections();
    let book_repo = SqliteBaseBookRepository::new(&conns.book_conn);
    let borrow_record_repo = SqliteBaseBorrowRecordRepository::new(&conns.borrow_record_conn);
    let reader_repo = SqliteBaseReaderRepository::new(&conns.reader_conn);
    let librarian_repo = SqliteBaseLibrarianRepository::new(&conns.librarian_conn);

    //主循环
    loop {
        match login(&reader_repo, &librarian_repo) {
            LoginResult::Quit => break,
            //注册完回到登录界面
            LoginResult::SignUp => {
                sign_up(&reader_repo, &librarian_repo);
            }
            LoginResult::ReaderUser(cur_reader) => {
                reader_contrl(&book_repo, &borrow_record_repo, &cur_reader);
            }
            LoginResult::LibrarianUser(_) => {
                librarian_contrl(&book_repo);
            }
        }
    }
}

/// sqlite 连接生成器：四个仓储共享 `resource/repo.db`，进程退出后仍然保留。
fn sqlite_connections() -> Connections {
    const DB_PATH: &str = "resource/repo.db";
    Connections {
        book_conn: open_sqlite_conn("图书", DB_PATH, new_book_table),
        borrow_record_conn: open_sqlite_conn("借阅记录", DB_PATH, new_borrow_record_table),
        reader_conn: open_sqlite_conn("读者", DB_PATH, new_reader_table),
        librarian_conn: open_sqlite_conn("管理员", DB_PATH, new_librarian_table),
    }
}

/// 构造一个 sqlite 连接；失败时打印原因并结束进程。
fn open_sqlite_conn<T>(
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
