fn main() {
    let arr: [u8; 3] = [1, 2, 3];
    let other_arr: [u8; 5] = [100; 5]; //Here we have x5 100 -> [100, 100, 100, 100, 100]
    println!("index: {}, length: {}", arr[0], other_arr.len());

    //Print structure of array and other objects 
    println!("{:?}", other_arr);
}