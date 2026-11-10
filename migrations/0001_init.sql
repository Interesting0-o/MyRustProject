CREATE TABLE IF NOT EXISTS books (
    bid VARCHAR(64) PRIMARY KEY,              -- 对应 String，作为主键（假设是 ISBN 或 UUID）
    name VARCHAR(255) NOT NULL,               -- 对应 String，书名
    author VARCHAR(255) NOT NULL,             -- 对应 String，作者
    price BIGINT NOT NULL,                    -- 对应 i64，价格（注意：这里存的是整数）
    num INT UNSIGNED NOT NULL DEFAULT 0       -- 对应 u32，库存数量，无符号整数
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE IF NOT EXISTS readers (
    account VARCHAR(64) PRIMARY KEY,          -- 对应 String，作为主键
    name VARCHAR(255) NOT NULL,               -- 对应 String，姓名
    hash_pwd BIGINT UNSIGNED NOT NULL         -- 对应 u64，密码摘要
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE IF NOT EXISTS librarian (
    account VARCHAR(64) PRIMARY KEY,          -- 对应 String，用户账号
    name VARCHAR(255) NOT NULL,               -- 对应 String 用户名
    hash_pwd BIGINT UNSIGNED NOT NULL         -- 对应 u64 用户密码的哈希值
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE IF NOT EXISTS borrow_record (
    br_id VARCHAR(64) PRIMARY KEY,            -- 对应 String，记录编号，全局唯一
    bid VARCHAR(64) NOT NULL,                 -- 对应 String，books.bid
    account VARCHAR(64) NOT NULL,             -- 对应 String，readers.account
    INDEX idx_borrow_record_bid (bid),
    INDEX idx_borrow_record_account (account)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
