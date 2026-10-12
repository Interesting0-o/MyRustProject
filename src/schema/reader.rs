use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Reader {
    /// 登录账号，同时是读者在仓储中的唯一标识（主键），创建后不可更改。
    pub account: String,
    pub hash_pwd: u64,
    pub name: String,
}
