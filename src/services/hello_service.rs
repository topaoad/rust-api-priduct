pub fn get_hello_message() -> String {
    "Hello, Actix!".to_string()
}

pub fn get_greeting(name: &str) -> String {
    format!("Hello, {}!", name)
}