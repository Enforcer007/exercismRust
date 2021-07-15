pub fn reply(message: &str) -> &str {
    let message = message.trim();
    let fine = message.len() == 0;
    let how_are_you = !fine && message.chars().last().unwrap() == '?';
    let yell_at_him =
        !fine && message.to_uppercase() == message && message.chars().any(|f| f.is_alphabetic());
    let calm_down = yell_at_him && how_are_you;

    match (how_are_you, yell_at_him, calm_down, fine) {
        (true, _, false, _) => "Sure.",
        (_, true, false, _) => "Whoa, chill out!",
        (_, _, true, _) => "Calm down, I know what I'm doing!",
        (_, _, _, true) => "Fine. Be that way!",
        _ => "Whatever.",
    }
}
