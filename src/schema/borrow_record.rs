/// 借阅记录：一条记录表示「某读者借走了某本书」。
///
/// - `br_id`：记录编号，全局唯一
/// - `bid`：被借图书的编号
/// - `account`：借阅者的账号
#[derive(Clone, Debug)]
pub struct BorrowRecord {
    pub br_id: String,
    pub bid: String,
    pub account: String,
}

#[derive(Debug)]
pub enum RemoveBorrowRecordError {
    NonExistedBRID,
}
