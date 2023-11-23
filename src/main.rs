use crate::router::MyLogin;

// main.rs
mod router;

fn main() {
    router::init_router();
    let login = router::Login::create("username".to_string(), "password".to_string());
    println!("username: {}, password: {}", login.username, login.password);
}
