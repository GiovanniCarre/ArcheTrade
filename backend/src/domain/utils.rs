pub fn can_be_symbol(query: &str) -> bool {
    query.chars().all(|c| c.is_ascii_uppercase()) && query.len() <= 5
}