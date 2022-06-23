pub fn raindrops(n: u32) -> String {
    let mut message: String = "".to_string();
    if n % 3 == 0 { message = format!("{}Pling", message); }
    if n % 5 == 0 { message = format!("{}Plang", message); }
    if n % 7 == 0 { message = format!("{}Plong", message); }
    if message == "" { return format!("{}", n) }
    message
}
