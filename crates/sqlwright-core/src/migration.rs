use std::{
    fs::{self, File},
    io::{self, Write},
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

pub(crate) fn write(dir: &Path, name: &str, sql: &str) -> io::Result<()> {
    let version = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(io::Error::other)?
        .as_secs();

    fs::create_dir_all(dir)?;

    let path = dir.join(format!("{version}_{name}.sql"));
    File::create_new(&path)?.write_all(sql.as_bytes())?;

    Ok(())
}
