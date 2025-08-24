pub fn get_hour(time: &str) -> String {
    if let Some(x) = time.split(":").next() {
        return x.to_string();
    } else {
        String::from("00")
    }
}
