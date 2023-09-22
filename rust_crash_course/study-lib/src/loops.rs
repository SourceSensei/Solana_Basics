fn main() {
    //For loop
    for i in 0..6 {
        println!("{}", i);
    }

    //While loop
    let mut a = 0;
    while a < 4 {
        println!("{}", a);
        a += 1;
        if a == 3 {
            println!("exit");
            break
        }
    }
}