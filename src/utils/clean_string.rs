pub fn get_clean_string<'a>(string: &'a str) -> Option<String> {
    if string.trim().is_empty() {
        return None;
    }

    Some(string.trim().to_string())
}
