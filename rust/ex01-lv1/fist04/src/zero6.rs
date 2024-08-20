// 사각형의 넓이 계산
// 사각형의 너비와 높이를 저장하는 구조체
// 사각형의 넓이를 계산하는 메서드 정의
// 구조체의 인스턴스를 생성하고 area 메서드를 호출해 넓이 출력
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl Rectangle {
    pub fn new(width: f64, height: f64) -> Rectangle {
        Rectangle { width, height }
    }
    pub fn area(&self) -> f64 {
        // return self.width * self.height;
        // 러스트에서는 return 을 생략하는게 관례임
        // return을 생략하면 마지막 구문이 반환됨
        // 반환되는 구문뒤에 세미콜론을 붙이면 안됨
        self.width * self.height
    }
}
