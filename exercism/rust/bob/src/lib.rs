pub fn reply(msg: &str) -> &str {
    let msg = msg.trim();
    let question = msg.ends_with("?");

    let yelling = msg.contains(char::is_alphabetic) && msg == msg.to_uppercase();
    // equivalent code
    // let yelling = msg.contains(|c: char| c.is_alphabetic()) && msg == msg.to_uppercase();

    match msg {
        m if m.is_empty() => "Fine. Be that way!",
        _ if question && yelling => "Calm down, I know what I'm doing!",
        _ if question => "Sure.",
        _ if yelling => "Whoa, chill out!",
        _ => "Whatever.",
    }
}
