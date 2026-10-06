use crate::{Book, Reader};

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
