fn main() {
    // Commenting here for the first time
    /*
        Siuuuuuuuuuu first variable
    */
    let name = "Cagan";
    let second_name = "Arda";
    let age = 21;
    println!("My first name is: {}", name);
    println!("My middle name is: {}. I am {}.", second_name, age);
    println!("Hello, world!");
    println!("I am learning Rust.");

    let mut x = 5;
    println!("Before: {}", x);

    x = 10;
    println!("After: {}", x);

    // BELOW GIVES ERROR BECAUSE y NEEDS mut keyword to be mutable
    // let y = 5;
    // println!("Before: {}", y);

    // y = 10;
    // println!("After: {}", y);

    // types
    let z: i32 = 32;
    let full_name: &str = "Cagan Arda Ozkan";
    let a: char = 'a';
    let is_learning: bool = true;
    let price: f64 = 19.99;

    // new line operator and combining data types
    print!("z is {}. Full name is {}. a is letter {}. is_learning? {}. Price is {}.\n", z, full_name, a, is_learning, price);

}
