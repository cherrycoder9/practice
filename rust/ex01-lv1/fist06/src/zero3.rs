// 트레이트 바운드와 다중 트레이드 구현
// 특정 타입이 여러 트레이트를 구현해야 하는 제네릭 함수를 정의할 수 있음
trait Drawable {
    fn draw(&self);
}

trait Resizable {
    fn resize(&mut self, scale: f64);
}

// 구조체 정의 및 트레이트 구현
struct Rectangle {
    width: f64,
    height: f64,
}

impl Drawable for Rectangle {
    fn draw(&self) {
        println!(
            "가로 {}, 세로 {}인 사각형 그리는 중",
            self.width, self.height
        )
    }
}

impl Resizable for Rectangle {
    fn resize(&mut self, scale: f64) {
        self.width *= scale;
        self.height *= scale;
        println!("사이즈 변환 완료: {}, {}", self.width, self.height);
    }
}

// 제네릭 함수: Drawable과 Resizable 트레이트를 모두 구현해야 함
fn manipulate_and_draw<T: Drawable + Resizable>(shape: &mut T) {
    shape.resize(0.5);
    shape.draw();
}

pub fn main() {
    let mut rect = Rectangle {
        width: 10.0,
        height: 5.0,
    };
    rect.resize(2.0);
    rect.draw();
    manipulate_and_draw(&mut rect);
}
