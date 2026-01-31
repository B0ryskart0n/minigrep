fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        eprintln!("Too few arguments. Expected PATTERN and at least one FILE.");
        std::process::exit(1);
    }

    let pattern = &args[1];
    let files = &args[2..];

    files
        .iter()
        .for_each(|file: &String| match std::fs::read_to_string(file) {
            Err(e) => eprintln!("Can't read contents of {file}: {e}"),
            Ok(contents) => search(&contents, pattern),
        });
}

fn search(contents: &String, pattern: &String) {
    contents.lines().for_each(|line| {
        if line.contains(pattern) {
            println!("{line}");
        }
    })
}
