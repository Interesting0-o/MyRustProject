//! 图书管理系统核心库。
//!
//! 自上而下分为四层，依赖方向单向朝内，下层不反向依赖上层：
//!
//! - [`schema`]：领域类型本身，纯数据，不依赖其他层。
//! - [`adapter`]：仓储 trait，描述领域对象需要哪些持久化操作。
//! - [`infrastructure`]：仓储的具体实现（内存 / txt），实现 [`adapter`] 里的 trait。
//! - [`service`]：业务用例（借书、还书、登录、注册），只依赖上面几层的抽象。
//!
//! 命令行界面不在这里——`src/cli/` 由二进制 crate 声明，属于组装根，
//! 直接读写 `stdin`/`stdout`，不应混进纯业务的 `service` 层。
//!
//! 各层的公开类型在 crate 根部统一再导出一份，调用方直接 `use book_manager::BookRepository`
//! 即可，不必关心它来自哪个子模块；子模块本身仍是公开的，需要精确路径时仍可深入引用。

pub mod adapter;
pub mod infrastructure;
pub mod schema;
pub mod service;
pub mod utils;

// ---- 领域类型 ----
pub use crate::schema::{Book, BorrowRecord, Librarian, Reader};

// ---- 仓储抽象：业务层只依赖这些 trait，不关心具体实现 ----
pub use crate::adapter::{
    book_repo::{AddBookError, BookRepository, FindBookError, UpdateBookError},
    borrow_record_repo::{BorrowRecordRepository, RemoveBorrowRecordError},
    librarian_repo::{
        AddLibrarianError, FindLibrarianError, LibrarianRepository, UpdateLibrarianError,
    },
    reader_repo::{AddReaderError, FindReaderError, ReaderRepository, UpdateReaderError},
};

// ---- 业务用例 ----
pub use crate::service::{
    borrow_book::{BorrowBookError, borrow_book},
    login::{LoginError, librarian_login, reader_login},
    return_book::{ReturnBookError, return_book},
    sign_up::{SignUpRes, librarian_sign_up, reader_sign_up},
};

// ---- 仓储实现：仅供组装根（二进制 crate）挑选具体仓储 ----

// 内存实现：数据只存在进程内，重启即丢失
pub use crate::infrastructure::in_memory::{
    in_memory_book_repository::InMemoryBookRepository,
    in_memory_borrow_record_repository::InMemoryBorrowRecordRepository,
    in_memory_librarian_repository::InMemoryLibrarianRepository,
    in_memory_reader_repository::InMemoryReaderRepository,
};

// txt 实现：数据落盘到 resource/*.txt
pub use crate::infrastructure::txt_base::{
    TXTBaseNewError, txt_base_book_repo::TxtBaseBookRepository,
    txt_base_librarian_repo::TxtBaseLibrarianRepository,
    txt_base_reader_repo::TXTBaseReaderRepository,
};
