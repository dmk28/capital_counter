fn count_capital_letters(s: &str) -> usize {
    s.chars()
        .filter(|c| c.is_ascii_uppercase())
        .count()
}





fn main() {
    let s = "Test Case Here";
    let count = count_capital_letters(s);
    println!("Capital Letters: {}", count);
}
