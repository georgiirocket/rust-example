fn main() {
    let s = String::from("hello world");

    let first_w = first_word(&s);

    println!("{}", first_w); //hello
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
