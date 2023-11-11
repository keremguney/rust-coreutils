use std::fs;
use std::io;

pub fn list_directories() -> Result<(), io::Error> {
    let dirs = match fs::read_dir("./") {
        Ok(dir) => dir,
        Err(e) => return Err(e),
    };

    for dir in dirs {
        println!("{}", dir.unwrap().path().display());
    }
    Ok(())
}
