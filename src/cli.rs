//! 命令行界面层：登录流程与各角色控制器。
//!
//! 只由 `main.rs` 声明，属于二进制 crate，不进入库的公开 API——
//! 这里直接读写 `stdin`/`stdout`，不应混进纯业务的 `service` 层。

pub mod librarian;
pub mod login;
pub mod reader;
pub mod sign_up;
