fn main() {

    // let value: i32 = 127;

    let i: i8 = 127_i8.wrapping_add(10i8);
    // i = 127 - 10
    println!("wrapping: {i}");
    
    let i: i8 = 127_i8.saturating_add(10i8);
    println!("saturated: {i}");
}
