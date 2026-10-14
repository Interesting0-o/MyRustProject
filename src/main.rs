mod cli;

use std::process;

use book_manager::{
    JSONBaseBookRepository, JSONBaseBorrowRecordRepository, JSONBaseLibrarianRepository,
    JSONBaseNewError, JSONBaseReaderRepository,
};

use crate::cli::{
    librarian::librarian_contrl,
    login::{LoginResult, login},
    reader::reader_contrl,
    sign_up::sign_up,
};

/// 默认使用的 json 仓储组：供主循环与各控制器共享。
struct Repositories {
    book: JSONBaseBookRepository,
    reader: JSONBaseReaderRepository,
    librarian: JSONBaseLibrarianRepository,
    borrow_record: JSONBaseBorrowRecordRepository,
}

fn main() {
    //仓库初始化：默认使用 json 仓储，数据落盘到 resource/json/*.json
    let repos = json_repositories();

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

/// json 仓库生成器：从 `resource/json/` 下加载数据，进程退出后仍然保留。
fn json_repositories() -> Repositories {
    Repositories {
        book: open_json_repo(
            "图书",
            "resource/json/books.json",
            JSONBaseBookRepository::new,
        ),
        reader: open_json_repo(
            "读者",
            "resource/json/readers.json",
            JSONBaseReaderRepository::new,
        ),
        librarian: open_json_repo(
            "管理员",
            "resource/json/librarians.json",
            JSONBaseLibrarianRepository::new,
        ),
        borrow_record: open_json_repo(
            "借阅记录",
            "resource/json/borrow_records.json",
            JSONBaseBorrowRecordRepository::new,
        ),
    }
}

/// 构造一个 json 仓储；失败时打印原因并结束进程。
fn open_json_repo<T>(
    name: &str,
    path: &str,
    open: impl FnOnce(&str) -> Result<T, JSONBaseNewError>,
) -> T {
    match open(path) {
        Ok(repo) => repo,
        Err(JSONBaseNewError::PathIsEmpty) => {
            eprintln!("{name}仓储路径为空：{path}");
            process::exit(1);
        }
        Err(JSONBaseNewError::OpenFailed(e)) => {
            eprintln!("打开{name}仓储文件失败：{path}：{e}");
            process::exit(1);
        }
        Err(JSONBaseNewError::ParseFailed) => {
            eprintln!("解析{name}仓储文件失败：{path}");
            process::exit(1);
        }
    }
}
