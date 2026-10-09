//! 基于 txt 文件的仓储实现：数据落盘到 `resource/*.txt`。

pub mod txt_base_book_repo;
pub mod txt_base_librarian_repo;
pub mod txt_base_reader_repo;

/// 所有 txt 仓储共用的构造错误。
pub enum TXTBaseNewError {
    /// 仓储路径为空。
    PathIsEmpty,
    /// 打开或创建仓储文件失败（目录不存在、权限不足等）。
    OpenFailed(std::io::Error),
}
