mod cli;

use std::process;

use book_manager::{
    TXTBaseBorrowRecordRepository, TXTBaseNewError, TXTBaseReaderRepository, TxtBaseBookRepository,
    TxtBaseLibrarianRepository,
};

use crate::cli::{
    librarian::librarian_contrl,
    login::{LoginResult, login},
    reader::reader_contrl,
    sign_up::sign_up,
};

fn main() {
    //仓库初始化：数据从 resource/txt 下的文件加载，进程退出后仍然保留
    let book = open_repo("图书", "resource/txt/books.txt", TxtBaseBookRepository::new);
    let reader = open_repo(
        "读者",
        "resource/txt/readers.txt",
        TXTBaseReaderRepository::new,
    );
    let librarian = open_repo(
        "管理员",
        "resource/txt/librarians.txt",
        TxtBaseLibrarianRepository::new,
    );
    let borrow_record = open_repo(
        "借阅记录",
        "resource/txt/borrow_records.txt",
        TXTBaseBorrowRecordRepository::new,
    );

    //主循环
    loop {
        match login(&reader, &librarian) {
            LoginResult::Quit => break,
            //注册完回到登录界面
            LoginResult::SignUp => {
                sign_up(&reader, &librarian);
            }
            LoginResult::ReaderUser(cur_reader) => {
                reader_contrl(&book, &borrow_record, &cur_reader);
            }
            LoginResult::LibrarianUser(_) => {
                librarian_contrl(&book);
            }
        }
    }
}

/// 构造一个 txt 仓储；失败时打印原因并结束进程。
fn open_repo<T>(
    name: &str,
    path: &str,
    open: impl FnOnce(&str) -> Result<T, TXTBaseNewError>,
) -> T {
    match open(path) {
        Ok(repo) => repo,
        Err(TXTBaseNewError::PathIsEmpty) => {
            eprintln!("{name}仓储路径为空：{path}");
            process::exit(1);
        }
        Err(TXTBaseNewError::OpenFailed(e)) => {
            eprintln!("打开{name}仓储文件失败：{path}：{e}");
            process::exit(1);
        }
    }
}
