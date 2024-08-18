// mod zero1;
// mod zero2;
// mod zero3;
// mod zero4;
mod zero5;

fn main() {
    // zero1::immutable_and_mutable_references();

    // zero2::immutable_and_mutable_references2();

    // let x = 5;
    // let y = 10;
    // let result = zero3::sum_values(&x, &y);
    // println!("x + y = {}", result);

    // let mut number = 8;
    // zero4::update_value(&mut number);
    // println!("number: {}", number);

    let x = 3;
    let y = 7;
    let mut sum = 0;
    zero5::print_values(&x, &y, &mut sum);
    println!("x: {}, y: {}, sum: {}", x, y, sum);
}
