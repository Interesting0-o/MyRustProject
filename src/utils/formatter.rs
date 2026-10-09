use crate::{Book, BorrowRecord, Librarian, Reader};

///用于txt存储的字符串格式化工具
///参数为一个Book的借用
pub fn format_book_for_storage(book: &Book) -> String {
    format!(
        "{name}|{bid}|{author}|{num}|{price}",
        name = book.name,
        bid = book.bid,
        author = book.author,
        num = book.num,
        price = book.price
    )
}

pub fn format_reader_for_storage(reader: &Reader) -> String {
    format!(
        "{name}|{account}|{hash_pwd}",
        name = reader.name,
        account = reader.account,
        hash_pwd = reader.hash_pwd
    )
}

pub fn format_librarian_for_storage(librarian: &Librarian) -> String {
    format!(
        "{name}|{account}|{hash_pwd}",
        name = librarian.name,
        account = librarian.account,
        hash_pwd = librarian.hash_pwd
    )
}

pub fn format_borrow_record_for_storage(record: &BorrowRecord) -> String {
    format!(
        "{br_id}|{account}|{bid}",
        br_id = record.br_id,
        account = record.account,
        bid = record.bid,
    )
}
