//! 读者端：菜单主循环与三个交互流程。

use std::io;

use book_manager::{
    BookRepository, BorrowBookError, BorrowRecordRepository, Reader, ReturnBookError, borrow_book,
    return_book,
};

/// 读者菜单主循环：只负责打印菜单、读选择、分发到具体流程。
pub fn reader_contrl<B, BR>(book_repo: &mut B, borrow_record_repo: &mut BR, reader: &Reader)
where
    B: BookRepository,
    BR: BorrowRecordRepository,
{
    let mut user_input = String::new();
    loop {
        user_input.clear();
        println!("[1]查询当前已借阅的图书\n[2]搜索对应名字的书籍并借阅\n[3]归还图书");
        io::stdin()
            .read_line(&mut user_input)
            .expect("输入流错误！！");

        match user_input.as_bytes()[0] {
            b'1' => list_borrowed_books(book_repo, borrow_record_repo, reader),
            b'2' => search_and_borrow(book_repo, borrow_record_repo, reader),
            b'3' => return_books(book_repo, borrow_record_repo, reader),
            _ => println!("请输入有效值"),
        }
    }
}

/// 查询并打印当前读者已借阅的图书。
fn list_borrowed_books<B, BR>(book_repo: &B, borrow_record_repo: &BR, reader: &Reader)
where
    B: BookRepository,
    BR: BorrowRecordRepository,
{
    let records = borrow_record_repo.find_records_by_account(&reader.account);
    if records.is_empty() {
        println!("当前无借阅书籍");
        return;
    }

    for (idx, br) in records.iter().enumerate() {
        //记录里的 bid 必然指向一本存在的书（书只会新增和更新，不会删除）
        let book = book_repo
            .find_book_by_bid(&br.bid)
            .expect("借阅记录指向的图书必然存在");
        println!("{}. {}", idx + 1, book.name);
    }
}

/// 按书名搜索，并让用户从结果里选一本借阅。
fn search_and_borrow<B, BR>(book_repo: &mut B, borrow_record_repo: &mut BR, reader: &Reader)
where
    B: BookRepository,
    BR: BorrowRecordRepository,
{
    let mut book_name = String::new();
    println!("请输入要搜索的书名:");
    io::stdin().read_line(&mut book_name).expect("输入流错误！");
    //read_line 会带上末尾换行，去掉后再精确匹配
    let book_name = book_name.trim();

    //find_book_by_name 返回拥有所有权的 Vec，查询结果不再借用 book_repo，
    //这样后面借书时才能可变借用 book_repo
    let Ok(qurry_res) = book_repo.find_book_by_name(book_name) else {
        println!("查询图书失败，请稍后重试");
        return;
    };

    //判断是否有查询结果
    if qurry_res.is_empty() {
        println!("无搜索结果");
        return;
    }

    println!("书籍搜索结果如下:");
    for (idx, book) in qurry_res.iter().enumerate() {
        println!(
            "{}. {}\n- 书籍编号:{}\n- 剩余数量:{}\n- 单价:{}.{:02}",
            idx + 1,
            book.name,
            book.bid,
            book.num,
            book.price / 100,
            book.price % 100
        );
    }

    while let Some(i) = pick_index(
        qurry_res.len(),
        "输入要借阅的书籍序号进行借阅，输入0退出借阅:",
    ) {
        match borrow_book(
            book_repo,
            borrow_record_repo,
            &reader.account,
            &qurry_res[i].bid,
        ) {
            Ok(()) => {
                println!("借阅成功！");
                break;
            }
            Err(BorrowBookError::BookNumIsZero) => {
                println!("该书籍已被借完，请重新输入");
                continue;
            }
            //bid 刚由同一个 book_repo 查出，必然存在
            Err(BorrowBookError::NonExistBID) => {
                unreachable!("bid 刚由同一 book_repo 查出，不可能不存在")
            }
            Err(BorrowBookError::RepositoryError(_)) => println!("查询图书失败，请稍后重试"),
        }
    }
}

/// 列出当前读者的借阅记录，并让用户选一条归还。
fn return_books<B, BR>(book_repo: &mut B, borrow_record_repo: &mut BR, reader: &Reader)
where
    B: BookRepository,
    BR: BorrowRecordRepository,
{
    let records = borrow_record_repo.find_records_by_account(&reader.account);
    if records.is_empty() {
        println!("当前无借阅书籍");
        return;
    }

    println!("当前已借阅书籍如下:");
    for (idx, br) in records.iter().enumerate() {
        //记录里的 bid 必然指向一本存在的书（书只会新增和更新，不会删除）
        let book = book_repo
            .find_book_by_bid(&br.bid)
            .expect("借阅记录指向的图书必然存在");
        println!("{}. {}\n- 书籍编号:{}", idx + 1, book.name, book.bid);
    }

    while let Some(i) = pick_index(
        records.len(),
        "输入要归还的书籍序号进行归还，输入0退出归还:",
    ) {
        let br = &records[i];
        match return_book(book_repo, borrow_record_repo, &br.bid, &reader.account) {
            Ok(()) => {
                println!("归还成功！");
            }
            //记录刚由同一个 reader.account 查出，必然匹配
            Err(ReturnBookError::NotFoundRecord) => {
                unreachable!("记录刚由同一 account 查出，不可能匹配不上")
            }
            //记录指向的图书必然存在（书只会新增和更新，不会删除）
            Err(ReturnBookError::NonExistBID) => {
                unreachable!("借阅记录指向的图书不可能不存在")
            }
            Err(ReturnBookError::RepositoryError(_)) => println!("查询图书失败，请稍后重试"),
        }
    }
}

/// 让用户从 `1..=len` 中选一个序号，返回对应的 0 基下标。
///
/// 输入 `0` 表示退出，返回 `None`；非法或越界的输入会提示后重新询问。
pub fn pick_index(len: usize, prompt: &str) -> Option<usize> {
    let mut index = String::new();
    loop {
        println!("{prompt}");
        index.clear();
        io::stdin().read_line(&mut index).expect("输入流错误！");

        //String转换成usize类型
        match index.trim().parse::<usize>() {
            Ok(0) => return None,
            //列表里展示的是从 1 开始的序号，所以要取 n-1
            Ok(n) if n <= len => return Some(n - 1),
            _ => {
                println!("请输入有效值！");
                continue;
            }
        }
    }
}
