use crate::{Book, Librarian, Reader};

pub enum GeneratorError {
    WrongParamType,
    WrongStrSplite,
}
pub fn generator_book_from_str(s: &str) -> Result<Book, GeneratorError> {
    let args: Vec<&str> = s.split('|').collect();
    let [name, bid, author, num, price] = args.as_slice() else {
        return Err(GeneratorError::WrongStrSplite);
    };

    let num = num
        .parse::<usize>()
        .map_err(|_| GeneratorError::WrongParamType)?;
    let price = price
        .parse::<i64>()
        .map_err(|_| GeneratorError::WrongParamType)?;

    Ok(Book {
        name: name.to_string(),
        bid: bid.to_string(),
        author: author.to_string(),
        num, // 字段和变量同名，简写
        price,
    })
}

pub fn generator_reader_from_str(s: &str) -> Result<Reader, GeneratorError> {
    let args: Vec<&str> = s.split('|').collect();
    let [name, account, hash_pwd] = args.as_slice() else {
        return Err(GeneratorError::WrongStrSplite);
    };

    let hash_pwd = hash_pwd
        .parse::<u64>()
        .map_err(|_| GeneratorError::WrongParamType)?;
    Ok(Reader {
        account: account.to_string(),
        name: name.to_string(),
        hash_pwd,
    })
}

pub fn generator_librarian_from_str(s: &str) -> Result<Librarian, GeneratorError> {
    let args: Vec<&str> = s.split('|').collect();
    let [name, account, hash_pwd] = args.as_slice() else {
        return Err(GeneratorError::WrongStrSplite);
    };

    let hash_pwd = hash_pwd
        .parse::<u64>()
        .map_err(|_| GeneratorError::WrongParamType)?;
    Ok(Librarian {
        account: account.to_string(),
        name: name.to_string(),
        hash_pwd,
    })
}
