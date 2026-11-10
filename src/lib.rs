//! 图书管理系统核心库。
//!
//! 自上而下分为四层，依赖方向单向朝内，下层不反向依赖上层：
//!
//! - [`schema`]：领域类型本身，纯数据，不依赖其他层。
//! - [`adapter`]：仓储 trait，描述领域对象需要哪些持久化操作。
//! - [`infrastructure`]：仓储的具体实现（sqlx/MySQL），实现 [`adapter`] 里的 trait。
//! - [`service`]：业务用例（借书、还书、登录、注册、更新），只依赖上面几层的抽象。
//!
//! 各层的公开类型在 crate 根部统一再导出一份，调用方直接 `use book_manager::AsyncBookRepository`
//! 即可，不必关心它来自哪个子模块；子模块本身仍是公开的，需要精确路径时仍可深入引用。

pub mod adapter;
pub mod api;
pub mod infrastructure;
pub mod schema;
pub mod service;

// ---- 领域类型 ----
pub use crate::schema::{Book, BorrowRecord, Librarian, Reader};

// ---- 仓储抽象：业务层只依赖这些 trait，不关心具体实现 ----
pub use crate::adapter::{
    BackendError,
    book_repo::{AddBookError, AsyncBookRepository, FindBookError, UpdateBookError},
    borrow_record_repo::{AsyncBorrowRecordRepository, RemoveBorrowRecordError},
    librarian_repo::{
        AddLibrarianError, AsyncLibrarianRepository, FindLibrarianError, UpdateLibrarianError,
    },
    reader_repo::{AddReaderError, AsyncReaderRepository, FindReaderError, UpdateReaderError},
    unit_of_work::{AsyncRepositoryGroup, AsyncUnitOfWork, UnitOfWorkError},
};

// ---- 业务用例 ----
pub use crate::service::{
    borrow_book::{BorrowBookError, async_borrow_book},
    login::{LoginError, async_librarian_login, async_reader_login},
    return_book::{ReturnBookError, async_return_book},
    sign_up::{SignUpRes, async_librarian_sign_up, async_reader_sign_up},
    update_book::{UpdateBookProfileError, async_update_book},
    update_reader::{UpdateReaderProfileError, async_update_reader},
};

// ---- 仓储实现：sqlx/MySQL ----
pub use crate::infrastructure::mysql_base::{
    MySqlUNewError, MySqlUnitOfWork, new_mysql_uow, run_migrations, with_mysql_uow,
};
