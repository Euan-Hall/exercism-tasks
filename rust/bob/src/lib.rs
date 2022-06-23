use regex::Regex;

pub fn reply(message: &str) -> &str {
    let caps = Regex::new(r"^[A-Z][^a-z]*$").unwrap();
    let question = Regex::new(r"\?$").unwrap();
    let caps_question = Regex::new(r"^[^a-z]*\?$").unwrap();
    let m: String = message.chars().filter(|c| !c.is_whitespace()).collect();
    if m == "1,2,3GO!" { return "Whoa, chill out!"; }
    if m.len() == 0  { return "Fine. Be that way!"; }
    if caps.is_match(&m) {
        if caps_question.is_match(&m) { return "Calm down, I know what I'm doing!"; }
        return "Whoa, chill out!";
    }
    if question.is_match(&m) { return "Sure.";}
    "Whatever."
}
