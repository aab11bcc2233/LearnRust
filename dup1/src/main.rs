use std::io;

fn main() {
    loop {
        let mut text = String::new();

        io::stdin()
            .read_line(&mut text)
            .expect("Failed to read line");

        let text = text.trim();
        if text == "q" {
            println!("bye-bye!");
            break;
        }
        println!("read line = {}", text);
    }
}
