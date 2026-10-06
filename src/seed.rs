//! 演示用种子数据：启动时构造一组已填好数据的仓储。
//!
//! 放在二进制 crate 而不是库的 `infrastructure`：填充数据要复用 `service`
//! 层的注册流程（密码散列由它负责），而基础设施层反过来依赖 `service` 会
//! 破坏分层方向。二进制本身是组装根，可以同时依赖两边。

use book_manager::{
    Book, BookRepository, BorrowRecordRepository, InMemoryBookRepository,
    InMemoryBorrowRecordRepository, InMemoryLibrarianRepository, InMemoryReaderRepository,
    LibrarianRepository, ReaderRepository, borrow_book, librarian_sign_up, reader_sign_up,
};

/// 一组已填入演示数据的仓储，供 `main` 直接使用。
pub struct SeededRepositories {
    pub reader: InMemoryReaderRepository,
    pub book: InMemoryBookRepository,
    pub borrow_record: InMemoryBorrowRecordRepository,
    pub librarian: InMemoryLibrarianRepository,
}

/// 构造并返回填好演示数据的仓储。
///
/// 图书的 `num` 是「剩余可借数」，所以借阅记录统一通过 [`borrow_book`] 生成，
/// 库存会被自动扣减，不会出现记录和库存对不上的情况。
pub fn seed_repositories() -> SeededRepositories {
    let mut reader = InMemoryReaderRepository::new();
    let mut book = InMemoryBookRepository::new();
    let mut borrow_record = InMemoryBorrowRecordRepository::new();
    let mut librarian = InMemoryLibrarianRepository::new();

    // 读者：账号 / 姓名 / 明文密码
    for (account, name, pwd) in [
        ("r001", "张三", "111111"),
        ("r002", "李四", "222222"),
        ("r003", "王五", "333333"),
    ] {
        if reader_sign_up(&mut reader, name, account, pwd).is_err() {
            unreachable!("种子读者账号重复：{account}");
        }
    }

    // 管理员：账号 admin / 密码 admin123
    if librarian_sign_up(&mut librarian, "图书管理员", "admin", "admin123").is_err() {
        unreachable!("种子管理员账号重复：admin");
    }

    // 图书：编号 / 书名 / 作者 / 单价（分） / 库存
    for (bid, name, author, price, num) in [
        ("B001", "红楼梦", "曹雪芹", 5900, 3),
        ("B002", "三体", "刘慈欣", 4200, 2),
        ("B003", "Rust 程序设计语言", "Steve Klabnik", 12800, 1),
    ] {
        let res = book.add(Book {
            bid: bid.to_string(),
            name: name.to_string(),
            author: author.to_string(),
            price,
            num,
        });
        if res.is_err() {
            unreachable!("种子图书编号重复：{bid}");
        }
    }

    // 借阅记录：r001 借走 B001；r002 借走 B003，正好把它借空（库存降为 0）
    for (account, bid) in [("r001", "B001"), ("r002", "B003")] {
        if borrow_book(&mut book, &mut borrow_record, account, bid).is_err() {
            unreachable!("种子借阅记录非法：{account} 借 {bid}");
        }
    }

    SeededRepositories {
        reader,
        book,
        borrow_record,
        librarian,
    }
}
