// 가변 참조를 사용해 값 변경하기
// 가변 참조를 인자로 받아, 해당 값을 2배로 변경
pub fn update_value(val: &mut i32) {
    *val *= 2;
}
