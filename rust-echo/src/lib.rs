use std::env;

pub fn take_input() -> String {
    let args: Vec<String> = env::args().collect();
    args[1].clone()
}

pub fn write_output(output: &str) {
    println!("{output}");
}
