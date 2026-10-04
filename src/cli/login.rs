//! 登录流程：选择角色、读取账号密码、校验并返回当前用户。

use std::io;

use book_manager::{
    Librarian, Reader,
    adapter::{librarian_repo::LibrarianRepository, reader_repo::ReaderRepository},
    service::login::{librarian_login, reader_login},
};

/// `login` 的结果：登录成功后的当前用户、用户选择注册，或用户选择退出。
pub enum LoginResult {
    ReaderUser(Reader),
    LibrarianUser(Librarian),
    /// 用户选择去注册；具体用哪个仓储注册由调用方决定。
    SignUp,
    Quit,
}

pub fn login<R, L>(reader_repo: &R, librarian_repo: &L) -> LoginResult
where
    R: ReaderRepository,
    L: LibrarianRepository,
{
    let mut login_state: String = String::new();
    let mut account: String = String::new();
    let mut pwd: String = String::new();

    loop {
        login_state.clear();

        println!("请选择登录方式:\n[1]Reader登录\n[2]Librarian登录\n[3]注册\n[4]exit");
        //输入
        io::stdin()
            .read_line(&mut login_state)
            .expect("输入流错误！");

        let input = login_state.as_bytes()[0];

        if input != b'1' && input != b'2' && input != b'3' && input != b'4' {
            println!("请输入有效值！");
            continue;
        } else if input == b'3' {
            break LoginResult::SignUp;
        } else if input == b'4' {
            break LoginResult::Quit;
        } else {
            account.clear();
            pwd.clear();

            //获取账号信息
            println!("请输入账号:");
            io::stdin().read_line(&mut account).expect("输入流错误");
            println!("请输入密码:");
            io::stdin().read_line(&mut pwd).expect("输入流错误");

            //根据
            if input == b'1' {
                if let Ok(r) = reader_login(reader_repo, account.trim(), pwd.trim()) {
                    break LoginResult::ReaderUser(r);
                } else {
                    println!("账号或密码错误请重新输入！");
                    continue;
                }
            } else if input == b'2' {
                if let Ok(l) = librarian_login(librarian_repo, account.trim(), pwd.trim()) {
                    break LoginResult::LibrarianUser(l);
                } else {
                    println!("账号或密码错误请重新输入！");
                    continue;
                }
            }
        }
    }
}
