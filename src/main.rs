fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        panic!("Too few arguments. Expected PATTERN and at least one FILE.")
    }

    let pattern = &args[1];
    let files = &args[2..];

    files.iter().for_each(|file: &String| {
        search(file, pattern);
    });
}

fn search(file: &String, pattern: &String) {
    match std::fs::read_to_string(file) {
        Err(e) => eprintln!("Can't read contents of {file}: {e}"),
        Ok(contents) => println!("{pattern}\n{contents}"),
    }
}
