use std::{env, fs};

fn take_input() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    args[1..=2].to_vec()
}

pub fn find() -> Result<(), std::io::Error> {
    let args = take_input();
    let dir = &args[0];
    let search = &args[1];

    let dirs = match fs::read_dir(dir) {
        Ok(dir) => dir,
        Err(e) => return Err(e),
    };

    for dir in dirs {
        let dir_string = dir.unwrap().path().display().to_string().rsplit_once('/').unwrap().1.to_string();
        if dir_string.contains(search) {
            println!("{}", dir_string);
        }
    }

    Ok(())
}
