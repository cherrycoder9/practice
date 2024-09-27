// 다양한 도형의 면적과 둘레를 계산하는 간단한 도형 계산기 구현

use std::io::{self, Write};

// 도형을 나타내는 열거형 정의
enum Shape {
    Circle(Circle),
    Rectangle(Rectangle),
    Triangle(Triangle),
}

// 원 구조체 정의
struct Circle {
    radius: f64, // 반지름
}

// 사각형 구조체 정의
struct Rectangle {
    width: f64,  // 너비
    height: f64, // 높이
}

// 삼각형 구조체 정의
struct Triangle {
    side_a: f64, // 첫 번째 변
    side_b: f64, // 두 번째 변
    side_c: f64, // 세 번째 변
}

// Shape에 대한 메소드 구현
impl Shape {
    // 면적 계산 메소드
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(c) => c.area(),    // 원 면적 계산
            Shape::Rectangle(r) => r.area(), // 사각형 면적 계산
            Shape::Triangle(t) => t.area(),  // 삼각형 면적 계산
        }
    }

    // 둘레 계산 메소드
    fn perimeter(&self) -> f64 {
        match self {
            Shape::Circle(c) => c.perimeter(),    // 원 둘레 계산
            Shape::Rectangle(r) => r.perimeter(), // 사각형 둘레 계산
            Shape::Triangle(t) => t.perimeter(),  // 삼각형 둘레 계산
        }
    }
}

// Circle 메소드 구현
impl Circle {
    // 면적 계산 메소드
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powi(2)
    }

    // 둘레 계산 메소드
    fn perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
}

// Rectangle 메소드 구현
impl Rectangle {
    // 면적 계산
    fn area(&self) -> f64 {
        self.width * self.height
    }

    // 둘레 계산
    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}

// Triangle 메소드 구현
impl Triangle {
    // 면적 계산 (헤론의 공식 사용)
    fn area(&self) -> f64 {
        let s = self.perimeter() / 2.0; // 반둘레 계산
        (s * (s - self.side_a) * (s - self.side_b) * (s - self.side_c)).sqrt()
    }

    // 둘레 계산
    fn perimeter(&self) -> f64 {
        self.side_a + self.side_b + self.side_c
    }
}

// 사용자로부터 입력을 받는 함수
fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().expect("출력 버퍼 플러시 실패");
    let mut input = String::new(); // 입력 저장 변수 초기화
    io::stdin().read_line(&mut input).expect("입력 읽기 실패");
    input.trim().to_string() // 입력 문자열 반환
}

pub fn main() {
    let shape: Shape = loop {
        println!("도형 계산기");
        println!("1. 원");
        println!("2. 사각형");
        println!("3. 삼각형");
        println!("4. 종료");

        let choice = read_input("원하는 도형의 번호를 입력하세요: ");

        let selected_shape = match choice.as_str() {
            "1" => {
                // 원 선택
                let radius_str = read_input("반지름 입력: ");
                // .parse::<f64>(): 문자열을 f64로 변환
                let radius = match radius_str.parse::<f64>() {
                    Ok(num) if num > 0.0 => num, // 유효한 숫자 확인
                    // 유효하지 않은 입력 처리
                    _ => {
                        println!("유효한 반지름을 입력하세요.");
                        continue; // 루프 계속
                    }
                };
                Shape::Circle(Circle { radius }) // 원 객체 생성
            }
            "2" => {
                // 사각형 선택
                let width_str = read_input("너비 입력: ");
                let width = match width_str.parse::<f64>() {
                    Ok(num) if num > 0.0 => num, // 유효한 숫자 확인
                    _ => {
                        println!("유효한 너비를 입력하세요.");
                        continue;
                    }
                };
                let height_str = read_input("높이 입력: ");
                let height = match height_str.parse::<f64>() {
                    Ok(num) if num > 0.0 => num,
                    _ => {
                        println!("유효한 높이를 입력하세요.");
                        continue;
                    }
                };
                Shape::Rectangle(Rectangle { width, height }) // 사각형 객체 생성
            }
            "3" => {
                let side_a_str = read_input("첫 번째 변의 길이 입력: ");
                let side_a = match side_a_str.parse::<f64>() {
                    Ok(num) if num > 0.0 => num,
                    _ => {
                        println!("유효한 첫 번째 변의 길이를 입력하세요.");
                        continue;
                    }
                };
                let side_b_str = read_input("두 번째 변의 길이 입력: ");
                let side_b = match side_b_str.parse::<f64>() {
                    Ok(num) if num > 0.0 => num,
                    _ => {
                        println!("유효한 두 번째 변의 길이를 입력하세요.");
                        continue;
                    }
                };
                let side_c_str = read_input("세 번째 변의 길이 입력: ");
                let side_c = match side_c_str.parse::<f64>() {
                    Ok(num) if num > 0.0 => num,
                    _ => {
                        println!("유효한 세 번째 변의 길이를 입력하세요.");
                        continue;
                    }
                };
                // 삼각형의 세 변이 삼각형 부등식을 만족하는지 확인
                if (side_a + side_b > side_c)
                    && (side_a + side_c > side_b)
                    && (side_b + side_c > side_a)
                {
                    Shape::Triangle(Triangle {
                        side_a,
                        side_b,
                        side_c,
                    }) // 삼각형 객체 생성
                } else {
                    println!("입력한 변의 길이로 삼각형을 만들 수 없습니다.");
                    continue;
                }
            }
            "4" => {
                println!("프로그램 종료");
                std::process::exit(0); // 프로그램 종료
            }
            _ => {
                // 잘못된 선택
                println!("유효한 옵션을 선택하세요.");
                continue; // 루프 계속
            }
        };

        break selected_shape; // 유효한 도형이 생성되면 루프 종료
    };

    // 선택한 도형의 면적과 둘레 계산
    let area = shape.area(); // 면적 계산
    let perimeter = shape.perimeter(); // 둘레 계산

    // 결과 출력
    println!("면적: {:.2}", area); // 면적 출력
    println!("둘레: {:.2}", perimeter); // 둘레 출력
    println!(); // 빈 줄 출력
}
