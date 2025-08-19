fn hcar1(a: String) -> String {
    let mut result = String::new();
    result.push_str("Hello, ");
    result.push_str(&a);
    result.push_str("!");
    result
}

fn main() {
    hcar1("World".to_string());
}
