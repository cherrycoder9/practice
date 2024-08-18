use zero1::{ex_for, ex_loop, ex_match, ex_while, if_else};
use zero2::check_number;
use zero3::sum_array;
use zero4::countdown;

mod zero1;
mod zero2;
mod zero3;
mod zero4;
fn main() {
    // if_else();
    // ex_match();
    // ex_while();
    // ex_for();
    // ex_loop();

    // check_number(-3)

    // let array: [i32; 7] = [1, 2, 3, 4, 5, 6, 7];
    // let result = sum_array(&array);
    // println!("result: {}", result);

    countdown(10);
}
