use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let nums: Vec<i32> = input
        .trim() // 공백 제거
        .split_whitespace() // 공백으로 분리
        .map(|x| x.parse::<i32>().expect("Invalid input"))// 문자열 -> 정수 변환 
        .collect(); // 벡터로 수집 
    
    let a = nums[0];
    let b = nums[1];
    
    println!("{}", a + b); 
}
