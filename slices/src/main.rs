fn main() {
    let w = String::from("What is my sentence");

    let first = first_word(&w);

    println!("First word: {first}"); 
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
