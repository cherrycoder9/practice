// 배열의 합 계산
// 정수 배열 numbers가 주어졌을때, 배열의 모든 요소를
// 더해 그 합을 반환하는 함수 작성, 반복문 사용
pub fn sum_array(numbers: &[i32]) -> i32 {
    let mut sum = 0;
    for e in numbers {
        sum += e;
    }
    return sum;
}
