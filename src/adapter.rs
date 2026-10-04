//! 仓储（Repository）抽象层。
//!
//! 这里只定义 trait，描述领域对象需要哪些持久化操作，不关心具体存储方式。
//! 具体实现位于 [`crate::infrastructure`]。

pub mod book_repo;
pub mod borrow_record_repo;
pub mod librarian_repo;
pub mod reader_repo;
