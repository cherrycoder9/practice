use std::f64::consts::PI;

// Traits와 제네릭 타입

// Trait 정의
trait Area {
    fn area(&self) -> f64;
}

// 구조체 정의 및 Trait 구현
struct Circle {
    radius: f64,
}

impl Area for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
}

struct Square {
    side: f64,
}

impl Area for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

// 제네릭 함수: Area 트레이트를 구현하는 모든 타입에 적용 가능함
fn print_area<T: Area>(shape: T) {
    println!("넓이는 {} 입니다.", shape.area());
}

pub fn main() {
    let circle = Circle { radius: 2.5 };
    let square = Square { side: 3.0 };
    print_area(circle);
    print_area(square);
}
