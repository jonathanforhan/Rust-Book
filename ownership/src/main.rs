
fn main() {

    let s = String::from("It's a brave new world");

    let word = first_word(&s);

    println!("{}", word);
}

fn first_word(s: &String) -> &str {

    for (i, n) in s.chars().into_iter().enumerate() {
        if n == ' ' {
            return &s[..i];
        }
    }
    &s
}
