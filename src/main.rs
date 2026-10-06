mod cli;
mod seed;

use crate::cli::{
    librarian::librarian_contrl,
    login::{LoginResult, login},
    reader::reader_contrl,
    sign_up::sign_up,
};

fn main() {
    //仓库初始化：用带演示数据的仓储，而不是空的 new()
    let mut repos = seed::seed_repositories();

    //主循环
    loop {
        match login(&repos.reader, &repos.librarian) {
            LoginResult::Quit => break,
            //注册完回到登录界面
            LoginResult::SignUp => {
                sign_up(&mut repos.reader, &mut repos.librarian);
            }
            LoginResult::ReaderUser(cur_reader) => {
                reader_contrl(&mut repos.book, &mut repos.borrow_record, &cur_reader);
            }
            LoginResult::LibrarianUser(_) => {
                librarian_contrl(&mut repos.book);
            }
        }
    }
}
