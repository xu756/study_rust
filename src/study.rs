use std::fs;
use std::fs::File;
use std::io::{Read, Write};

#[derive(Debug)]
struct User {
    name: String,
    age: u8,
}

impl User {
    fn new(name: String, age: u8) -> Self {
        Self { name, age }
    }
    fn get_name(&self) -> &String {
        &self.name
    }
    fn set_name(&mut self, name: String) {
        self.name = name
    }
}
#[test]
fn struct_test() {
    let mut user = User::new("Jack".to_string(), 18);
    println!("age: {}", user.age);
    println!("name: {}", user.get_name());
    user.set_name("Tom".to_string());
    println!("new name: {}", user.get_name());
}

#[test]
fn enum_test() {
    // enum Color {
    //     Red,
    //     Green,
    //     Blue,
    //     New(u8, u8, u8),
    // }
    // let color = Color::New(255, 0, 0);
    // match color {
    //     Color::Red => println!("red"),
    //     Color::Green => println!("green"),
    //     Color::Blue => println!("blue"),
    //     Color::New(r, g, b) => println!("rgb: {}, {}, {}", r, g, b),
    // }
}

#[test]
fn file_open_test() {
    match File::open("hello.txt") {
        Ok(mut file) => {
            // 按行读取
            let mut s = String::new();
            file.read_to_string(&mut s).unwrap();
            println!("{}", s);
        }
        Err(err) => {
            println!("{:?}", err)
        }
    }
}

#[test]
fn file_create_test() {
    let f = fs::File::create("hello.txt");
    match f {
        Ok(mut file) => {
            file.write(b"hello world").unwrap();
            println!("create file success");
        }
        Err(err) => {
            println!("{:?}", err)
        }
    }
}

#[test]
fn file_io_test() {
    // let args = std::env::args();
    // for arg in args {
    //     println!("{}", arg);
    // }
}
