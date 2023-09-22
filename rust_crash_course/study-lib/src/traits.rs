fn main() {
    let name = String::from("Bird");
    let bird = Bird { name, attacks: 5};
    bird.print_name();
    println!("{} {}", bird.can_fly(), bird.is_animal());
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

impl Animal for Bird {
    fn can_fly(&self) -> bool {
        true
    }
    fn is_animal(&self) -> bool {
        false
    }
}

trait Animal {
    fn can_fly(&self) -> bool;
    fn is_animal(&self) -> bool {
        true
    }
}