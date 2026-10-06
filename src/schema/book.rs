#[derive(Clone, Debug)]
pub struct Book {
    pub author: String,
    pub bid: String,
    pub name: String,
    pub price: i64,
    pub num: usize,
}

pub enum AddBookError {
    BIDAlreadyExists,
    /// 持久化到文件失败（磁盘满、权限不足等）。
    IoError(std::io::Error),
}

#[derive(Debug)]
pub enum UpdateBookError {
    NonExistedBID,
    /// 持久化到文件失败（磁盘满、权限不足等）。
    IoError(std::io::Error),
}
