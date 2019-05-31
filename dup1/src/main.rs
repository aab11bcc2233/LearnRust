use std::io;
use std::collections::HashMap;

fn main() {
    let mut counts: HashMap<String, u32> = HashMap::new();

    loop {
        let mut text = String::new();

        io::stdin()
            .read_line(&mut text)
            .expect("Failed to read line");

        let text = text.trim().to_string();
        if text == "q" {
            println!("input done!");
            break;
        }

        let count = counts.entry(text).or_insert(0);
        *count += 1;
    }

    for (line, count) in counts {
        if count > 1 {
            println!("{l:width$}{n}", l=line, width=4, n=count);
        }
    }
}
