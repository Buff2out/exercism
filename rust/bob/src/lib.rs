pub fn reply(message: &str) -> &str {
    let message = message.trim();
    if message.is_empty() {
        return "Fine. Be that way!";
    }
    if message.chars().any(char::is_alphabetic) && message.to_uppercase() == message {
        if message.ends_with("?") {
            return "Calm down, I know what I'm doing!";
        }
        return "Whoa, chill out!";
    }
    if message.ends_with("?") {
        return "Sure.";
    }
    return "Whatever.";
}
