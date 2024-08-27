// Trait 상속 및 제네릭 연산
// Shape 이란 기본 트레이트를 정의하고 이 트레이트에 area()와 perimeter() 두개의 메서드 선언
// Circle과 Rectangle 구조체를 생성하고 Shape 트레이트를 구현
// AreaComparable 이라는 새로운 트레이트를 정의하고 Shape 트레이트를 상속받고
// 두 도형의 면적을 비교하는 compare_area()메서드 추가
// 이 메서드는 두 도형의 면적이 동일하면 true 반환, 그렇지 않으면 false 반환
// 제네릭을 사용해 compare_area()가 Shape 트레이트를 구현하는 모든 타입에 대해 작동하도록 할 것

use std::f64::consts::PI;

// 기본 Shape 트레이트 정의
trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

// Circle 구조체 정의 및 Shape 트레이트 구현
struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
    fn perimeter(&self) -> f64 {
        2.0 * PI * self.radius
    }
}

// Rectangle 구조체 정의 및 Shape 트레이트 구현
struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}

// AreaComparable 트레이트 정의 및 Shape 트레이트 상속
trait AreaComparable: Shape {
    fn compare_area<T: Shape>(&self, other: &T) -> bool {
        self.area() == other.area()
    }
}

// Circle 구조체에 대해 AreaComparable 트레이트 구현
impl AreaComparable for Circle {}

// Rectangle 구조체에 대해 AreaComparable 트레이트 구현
impl AreaComparable for Rectangle {}

pub fn main() {
    let circle = Circle { radius: 3.0 };
    let rectangle = Rectangle {
        width: 3.0,
        height: 3.0,
    };

    let result = circle.compare_area(&rectangle);
    println!("원과 사각형의 넓이가 동일한가?: {}", result);

    let larger_rectangle = Rectangle {
        width: 6.0,
        height: 1.5,
    };
    let result = circle.compare_area(&larger_rectangle);
    println!("원과 더 큰 사각형 넓이가 동일한가?: {}", result);
}
