use regex::Regex;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        panic!("Too few arguments. Expected PATTERN and at least one FILE.")
    }

    let pattern = &args[1];
    let re = Regex::new(&pattern).expect("Can't create regex from given PATTERN");

    let files = &args[2..];

    files.iter().for_each(|file: &String| {
        search(file, &re);
    });
}

fn search(file: &String, re: &Regex) {
    match std::fs::read_to_string(file) {
        Err(e) => eprintln!("Can't read contents of {file}: {e}"),
        Ok(contents) => {
            contents.lines().for_each(|line| {
                if re.is_match(line) {
                    println!("{line}");
                }
            });
        }
    }
}
