fn main() {
    let name = String::from("Bird");
    let bird = Bird { name, attacks: 5};
    bird.print_name();
}

struct Bird {
    name: String,
    attacks: u64
}

impl Bird {
    fn print_name(&self) {
        println!("{}", self.name);
    }
}