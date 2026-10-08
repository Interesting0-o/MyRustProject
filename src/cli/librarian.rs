//! 管理员端：菜单主循环与三个交互流程。

use std::io;

use book_manager::{AddBookError, Book, BookRepository, FindBookError, UpdateBookError};

/// 管理员菜单主循环：只负责打印菜单、读选择、分发到具体流程。
pub fn librarian_contrl<B>(book_repo: &mut B)
where
    B: BookRepository,
{
    let mut choice = String::new();
    loop {
        println!("[1]根据书籍名称查询数据\n[2]添加图书\n[3]变更图书数目");

        choice.clear();
        io::stdin().read_line(&mut choice).expect("输入流错误!!");

        match choice.trim().parse::<usize>() {
            Ok(1) => search_books(book_repo),
            Ok(2) => add_book(book_repo),
            Ok(3) => update_book_num(book_repo),
            _ => println!("请输入有效值!"),
        }
    }
}

/// 按书名查询并打印所有同名图书。
fn search_books<B>(book_repo: &B)
where
    B: BookRepository,
{
    let book_name = read_line("请输入书籍名称:");
    let Ok(query_res) = book_repo.find_book_by_name(&book_name) else {
        println!("查询图书失败，请稍后重试");
        return;
    };

    if query_res.is_empty() {
        println!("无搜索结果");
        return;
    }

    for (idx, book) in query_res.iter().enumerate() {
        println!(
            "{}. {}\n- 作者:{}\n- 当前剩余数量:{}\n- 书籍编号:{}\n售价:{}.{:02}\n",
            idx + 1,
            book.name,
            book.author,
            book.num,
            book.bid,
            book.price / 100,
            book.price % 100,
        );
    }
}

/// 读取新书信息并入库。
fn add_book<B>(book_repo: &mut B)
where
    B: BookRepository,
{
    let name = read_line("请输入书名:");
    let bid = read_line("请输入书籍编号:");
    let author = read_line("请输入书籍作者:");
    let price = read_price("请输入书籍价格(元，最多两位小数):");
    let num = read_usize("请输入书籍数目:");

    let book = Book {
        name,
        bid,
        author,
        price,
        num,
    };

    match book_repo.add(book) {
        Ok(()) => println!("添加成功!"),
        //bid 是主键，重复时 add 会拒绝，不能覆盖已有图书
        Err(AddBookError::BIDAlreadyExists) => println!("书籍编号已存在，添加失败!"),
        Err(AddBookError::IoError(e)) => println!("图书添加失败，写入文件出错: {e}"),
        Err(AddBookError::RepositoryLockError) => println!("错误的写入时机"),
    }
}

/// 按编号找到图书，把它的剩余数量改成用户输入的新值。
fn update_book_num<B>(book_repo: &mut B)
where
    B: BookRepository,
{
    let book_bid = read_line("请输入要变更数目的书籍编号:");

    //先取出克隆副本，展示当前数量；确认后再整体写回
    let mut book = match book_repo.find_book_by_bid(&book_bid) {
        Ok(book) => book,
        Err(FindBookError::NoResult) => {
            println!("未找到该编号的图书");
            return;
        }
        Err(_) => {
            println!("查询图书失败，请稍后重试");
            return;
        }
    };
    println!("当前《{}》剩余数量:{}", book.name, book.num);

    book.num = read_usize("请输入新的书籍数目:");

    match book_repo.update(book) {
        Ok(()) => println!("变更成功!"),
        //bid 刚由同一个 book_repo 查出，必然存在
        Err(UpdateBookError::NonExistedBID) => {
            unreachable!("bid 刚由同一 book_repo 查出，不可能不存在")
        }
        Err(UpdateBookError::IoError(e)) => println!("变更失败，写入文件出错: {e}"),
        Err(UpdateBookError::RepositoryLockError) => println!("错误的写入时机"),
    }
}

/// 打印提示并读取一行，返回去掉首尾空白后的内容。
fn read_line(prompt: &str) -> String {
    println!("{prompt}");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("输入流错误!!");
    input.trim().to_string()
}

/// 反复询问，直到读到合法的 `usize`。
fn read_usize(prompt: &str) -> usize {
    loop {
        match read_line(prompt).parse::<usize>() {
            Ok(n) => return n,
            Err(_) => println!("请输入有效值!"),
        }
    }
}

/// 反复询问，直到读到合法的价格（以「分」为单位）。
fn read_price(prompt: &str) -> i64 {
    loop {
        match str2price(&read_line(prompt)) {
            Some(p) => return p,
            None => println!("请输入有效值!"),
        }
    }
}

/// 把 "12.34" 这样的价格字符串转成以「分」为单位的 i64（"12.34" -> 1234）。
///
/// 只接受非负十进制数：整数部分必填，小数部分最多两位。
/// 出现其他字符、多个小数点、超过两位小数或数值溢出时返回 `None`。
fn str2price(s: &str) -> Option<i64> {
    let s = s.trim();

    let mut int_part = 0_i64;
    let mut frac = 0_i64;
    //小数第一位的权重是 10 分，第二位是 1 分；减到 0 说明已经两位了
    let mut frac_scale = 10_i64;
    let mut has_digit = false;
    let mut seen_dot = false;

    for chr in s.chars() {
        match chr {
            '0'..='9' => {
                let digit = chr.to_digit(10).expect("已匹配到数字") as i64;
                has_digit = true;
                if seen_dot {
                    if frac_scale == 0 {
                        return None; //小数超过两位
                    }
                    frac += digit * frac_scale;
                    frac_scale /= 10;
                } else {
                    //整数部分整体放大 100 倍，再加上小数部分即为「分」
                    int_part = int_part.checked_mul(10)?.checked_add(digit)?;
                }
            }
            '.' if !seen_dot => seen_dot = true,
            _ => return None,
        }
    }

    if !has_digit {
        return None;
    }
    int_part.checked_mul(100)?.checked_add(frac)
}
