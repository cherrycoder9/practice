mod zero1;
mod zero2;
mod zero3;

fn main() {
    // zero1::immutable_and_mutable_references();
    // zero2::immutable_and_mutable_references2();
    let x = 5;
    let y = 10;
    let result = zero3::sum_values(&x, &y);
    println!("x + y = {}", result)
}
