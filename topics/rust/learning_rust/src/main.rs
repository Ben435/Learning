fn main() {
    let sentence = "hello world";

    let word = first_word(&sentence);

    println!("Got: '{}' -> '{}'", sentence, word);
}

fn first_word(string: &str) -> &str {
    for (i, &s) in string.as_bytes().iter().enumerate() {
        if s == b' ' {
            return &string[..i];
        }
    }

    return &string[..];
}