use std::{
    fs::File,
    io::{self, Read},
    path::PathBuf,
};

pub fn read_stdin() -> io::Result<String> {
    let mut buffer = Vec::new();
    let mut stdin = io::stdin();
    stdin.read_to_end(&mut buffer)?;

    Ok(String::from_utf8(buffer).unwrap())
}

pub fn read_path(path: &PathBuf) -> io::Result<String> {
    let mut buffer = String::new();
    let mut file = File::open(path)?;
    file.read_to_string(&mut buffer)?;

    Ok(buffer)
}
