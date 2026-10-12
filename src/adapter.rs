//! 仓储（Repository）抽象层。
//!
//! 这里只定义 trait，描述领域对象需要哪些持久化操作，不关心具体存储方式。
//! 具体实现位于 [`crate::infrastructure`]。

pub mod book_repo;
pub mod borrow_record_repo;
pub mod librarian_repo;
pub mod reader_repo;

/// 后端（仓储实现）故障的载体：连接中断、IO、序列化、锁中毒等，全部归到这一个不透明类型。
///
/// 端口不关心「怎么坏的」，各实现把底层错误直接装箱交给上层；
/// 新增后端不需要在错误枚举里加新变体，调用方也永远不用认识 `sqlx::Error`
/// 之类的具体类型。
pub type BackendError = Box<dyn std::error::Error + Send + Sync + 'static>;
