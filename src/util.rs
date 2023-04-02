use std::{io::{self, Write, BufRead}, path::Path};

use crate::transferdata;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn prompt(message: &str) -> Result<String> {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    stdout.write_all(message.as_bytes())?;
    stdout.flush()?;

    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    let mut line = String::new();
    stdin.read_line(&mut line)?;
    Ok(line)
}

pub async fn write_cache<P, S>(cp: P, key: i64, b: S) -> Result<()>
where
    P: AsRef<Path>,
    S: Sized + serde::Serialize,
{
    Ok({
        cacache::write(cp, key.to_string(), bincode::serialize(&b).unwrap()).await?;
    })
}

pub async fn find_match_file_query<P>(cp: P, t: &mut Vec<transferdata::WebSearchFileData>, query: &String) -> Result<()>
where
    P: AsRef<Path>,
{
    let path = cp.as_ref();
    let mut iteration_cache = cacache::list_sync(path);
    while let Some(item) = iteration_cache.next() {
        if !item.is_err() {
            let ret_doc = cacache::read(path, item.unwrap().key).await?;
            let doc: transferdata::WebSearchFileData = bincode::deserialize(&ret_doc).unwrap();

            if !doc.file_name.to_lowercase().find(query.to_lowercase().as_str()).is_none() {
                t.push(doc);
            }
        }
    }

    drop(path);
    Ok(())
}