use crate::{
    AddReaderError, Reader, ReaderRepository, UpdateReaderError,
    adapter::reader_repo::FindReaderError,
    utils::{formatter::format_reader_for_storage, generator::generator_reader_from_str},
};
use std::{
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, Write},
};

pub struct TXTBaseReaderRepository {
    readers: Vec<Reader>,
    pk_record: usize,
}
const READERS_PATH: &str = "resource/readers.txt";

impl TXTBaseReaderRepository {
    /// 构造仓储，并把 `resource/readers.txt` 里已有的读者读进来。
    ///
    /// 构造不属于 [`ReaderRepository`] 契约，由各个实现自行提供。
    pub fn new() -> Self {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .truncate(false)
            .create(true)
            .open(READERS_PATH)
            .unwrap_or_else(|e| panic!("无法打开文件{e}"));

        let readers: Vec<Reader> = BufReader::new(file)
            .lines()
            .map_while(Result::ok)
            .filter_map(|l| generator_reader_from_str(&l).ok())
            .collect();
        Self {
            pk_record: readers.len(),
            readers,
        }
    }

    /// 把内存里的全部读者信息原子地写回文件。
    ///
    /// 先写同目录下的临时文件，`sync_all` 落盘后再 `rename` 覆盖正式文件：
    /// 中途失败或崩溃时正式文件保持旧内容，不会留下半截数据。
    fn persist(&self) -> std::io::Result<()> {
        let tmp_path = format!("{READERS_PATH}.tmp");
        let mut file = File::create(&tmp_path)?;
        for reader in &self.readers {
            writeln!(file, "{}", format_reader_for_storage(reader))?;
        }
        file.sync_all()?;
        fs::rename(&tmp_path, READERS_PATH)
    }
}

impl ReaderRepository for TXTBaseReaderRepository {
    fn add(&self, _reader: Reader) -> Result<(), AddReaderError> {
        todo!();
    }

    fn update(&self, _reader: Reader) -> Result<(), UpdateReaderError> {
        todo!();
    }
    fn find_reader_by_name(&self, _name: &str) -> Result<Vec<Reader>, FindReaderError> {
        todo!();
    }
    fn find_reader_by_account(&self, _account: &str) -> Result<Reader, FindReaderError> {
        todo!();
    }
}
