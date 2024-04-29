pub fn reply(message: &str) -> &str {
   let message = message.trim();

    if message.is_empty() {
        "Fine. Be that way!"
    } else if message.chars().all(|c| c.is_ascii_uppercase()) && message.contains('?') {
        "Calm down, I know what I'm doing!"
    } else if message.chars().all(|c| c.is_ascii_uppercase()) {
        "Whoa, chill out!"
    } else if message.ends_with('?') {
        "Sure."
    } else {
        "Whatever."
    }
}
