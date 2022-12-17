#[derive(Debug)]
struct Jean {
    name: String,
    last_name: String,
    age: u8,
}

impl Jean {
    fn new() -> Jean {
        Jean {
            name: String::from("Jean"),
            last_name: String::from("Kirchner"),
            age: 33,
        }
    }
}

fn main() {
    let jean = Jean::new();

    println!("{:?}", jean);
}
