use std::fs::File;
use std::{env, io};
use std::io::BufRead;
use std::path::{Path, PathBuf};

pub fn read_lines(filename: impl AsRef<Path> + Clone) -> Result<io::Lines<io::BufReader<File>>, String> {
    let file = File::open(filename.clone())
        .map_err(|_| format!("Can't find file at {:?}", absolute_path(filename)))?;
    Ok(io::BufReader::new(file).lines())
}

pub fn absolute_path(path: impl AsRef<Path>) -> io::Result<PathBuf> {
    let path = path.as_ref();

    let absolute_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        env::current_dir()?.join(path)
    };

    Ok(absolute_path)
}