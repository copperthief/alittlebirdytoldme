use std::fs;

fn main() {
    let mut file = fs::read_to_string("../res/quotes.txt")
        .expect("could not read quotes file");

    println!("{quotes_file}");
}

fn load(File file) {
    if let Ok(lines) = read_lines("../res/quotes") {
        for line in lines {
            let chars = lines.chars();
            if chars[0] == 'Q' {
                
            }
        }
    }
}

fn add(String file, String quote, String author, String time) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("../res/quotes.txt")
        .unwrap();


}
