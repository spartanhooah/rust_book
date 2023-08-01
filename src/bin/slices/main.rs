fn main() {
    let input = String::from("Hello, world!");
    let first = first_word(&input);

    println!("{}", first)
}

fn first_word(input: &str) -> &str {
    let bytes = input.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &input[..i];
        }
    }

    &input
}
