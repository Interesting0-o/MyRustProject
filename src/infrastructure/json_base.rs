pub mod json_base_book_repo;
pub mod json_base_reader_repo;
/// 所有 json 仓储共用的构造错误。
pub enum JSONBaseNewError {
    /// 仓储路径为空。
    PathIsEmpty,
    /// 打开或创建仓储文件失败（目录不存在、权限不足等）。
    OpenFailed(std::io::Error),
    ParseFailed,
}
