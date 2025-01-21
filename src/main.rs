use  std::io;

fn main() {
    println!("Enter your Weight (kg):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let weight = input.trim();

    println!("{},kg", weight);
    dbg!(weight);

    print!("Input: {}", input);
    let mars_weight = calculate_weight_on_mars(100.0);
    println!("Weight on Mars: {}kg", mars_weight);

}


fn calculate_weight_on_mars(weight: f32) -> f32{
    (weight / 9.81) * 3.711
}


// Ownership 
// 1. Each value in Rust is owned by a variable.
// 2. When the owner goes out of scope, the value will be deallocated.
// 3. There can only be ONE owner at a time. 