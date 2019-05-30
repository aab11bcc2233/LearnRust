use std::io;

fn main() {
    loop {
        let mut text = String::new();

        io::stdin()
            .read_line(&mut text)
            .expect("Failed to read line");

        if text.as_str().eq("q") {
            println!("q");
            break;
        }
        println!("read line = {}", text);
    }
}
