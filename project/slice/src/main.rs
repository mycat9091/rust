fn main() {
    let s: String = String::from("中国");
    let a = first_word(&s);
    println!("{}", a);
}
fn first_word(s: &String) -> &str {
    &s[..]
}
