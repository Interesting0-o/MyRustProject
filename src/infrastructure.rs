//基于内存管理的仓库
pub mod in_memory;

//基于txt读取的仓库
pub mod txt_base;

//基于json读取的仓库
pub mod json_base;

use crate::BackendError;

/// 把锁中毒统一包装成仓储故障。
///
/// 真实的 `PoisonError` 持有带生命周期的 guard，无法装箱成 `'static`，
/// 这里用一个不带 guard 的等价错误代替。所有用锁的仓储实现共用。
pub(crate) fn lock_poisoned() -> BackendError {
    Box::new(std::sync::PoisonError::new(()))
}
