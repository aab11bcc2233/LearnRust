fn main() {
    println!("{}", first_word(&String::from("Hello, world!")));
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if b' ' == item {
            return &s[..i];
        }
    }

    &s[..]
}
