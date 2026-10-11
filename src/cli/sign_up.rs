use std::io;

use book_manager::{
    LibrarianRepository, ReaderRepository, SignUpRes, librarian_sign_up, reader_sign_up,
};

pub fn sign_up<R, L>(reader_repo: &R, librarian_repo: &L)
where
    R: ReaderRepository,
    L: LibrarianRepository,
{
    let mut identity = String::new();

    let mut account = String::new();
    let mut pwd = String::new();
    let mut name = String::new();
    loop {
        identity.clear();
        println!("请选择你要注册的用户身份\n[1]读者\n[2]图书管理员\n[3]返回登录");
        io::stdin().read_line(&mut identity).expect("输入流错误！");

        match identity.trim().parse::<usize>() {
            Ok(n) if n == 1 || n == 2 => {
                account.clear();
                println!("请输入账号:");
                io::stdin().read_line(&mut account).expect("输入流错误!!");

                name.clear();
                println!("请输入称呼:");
                io::stdin().read_line(&mut name).expect("输入流错误!!");

                pwd.clear();
                println!("请输入密码:");
                io::stdin().read_line(&mut pwd).expect("输入流错误!!");

                let mut retry_pwd = String::new();
                retry_pwd.clear();
                println!("请再次输出密码:");
                io::stdin().read_line(&mut retry_pwd).expect("输入流错误!!");
                if retry_pwd != pwd {
                    println!("前后两次密码不一致，请重试");
                    continue;
                }

                let res = if n == 1 {
                    reader_sign_up(reader_repo, name.trim(), account.trim(), pwd.trim())
                } else {
                    librarian_sign_up(librarian_repo, name.trim(), account.trim(), pwd.trim())
                };

                match res {
                    Ok(()) => {
                        println!("注册成功，请登录");
                        break;
                    }
                    Err(SignUpRes::AccountAlreadyExists) => {
                        println!("当前账户已被注册，请重试");
                        continue;
                    }
                    Err(SignUpRes::RepositoryError) => {
                        println!("注册失败，请稍后重试");
                        continue;
                    }
                }
            }
            Ok(3) => break,
            _ => {
                println!("请输入有效值！");
                continue;
            }
        }
    }
}
