use std::env;
use std::fs;

pub fn take_input() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    args[1..].to_vec()
}

pub fn concat_files(args: Vec<String>) {
    let args = args.into_iter().map(|file| fs::read_to_string(file).expect("failed to read file")).collect::<Vec<String>>();
    println!("{}", args.join(""));
}
