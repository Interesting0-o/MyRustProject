//! 基于内存的仓储实现：数据只存在进程内，重启即丢失。

pub mod in_memory_book_repository;
pub mod in_memory_borrow_record_repository;
pub mod in_memory_librarian_repository;
pub mod in_memory_reader_repository;
