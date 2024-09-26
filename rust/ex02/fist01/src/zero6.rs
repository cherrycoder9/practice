// 학생들의 이름과 각 과목별 점수를 저장하고 관리할 수 있는 프로그램 작성

use std::io::{self, Write};

// 학생 정보를 저장하는 구조체 정의
struct Student {
    name: String,
    math: i32,
    science: i32,
    english: i32,
}

// 학생의 총점과 평균을 계산하는 함수
fn calculate_total_and_average(student: &Student) -> (i32, f32) {
    let total = student.math + student.science + student.english;
    let average = total as f32 / 3.0;
    (total, average) // 총점과 평균 반환
}

// 학생을 검색해 정보를 출력하는 함수
fn search_student(students: &Vec<Student>, name: &str) {
    for student in students {
        if student.name == name {
            // 이름이 일치하는 학생 찾기
            // 총점과 평균 계산
            let (total, average) = calculate_total_and_average(student);
            println!("학생 이름: {}", student.name);
            println!("수학 점수: {}", student.math);
            println!("과학 점수: {}", student.science);
            println!("영어 점수: {}", student.english);
            println!("총점: {}", total);
            // 콜론(:)은 포맷 지정자의 시작을 알리는 기호
            // .2는 소수점 이하 두 자리까지 표시하란 의미
            // f 생략 가능. 기본적으로 변수 타입이 f32 또는 f64인 경우 소수점 형식으로 출력됨
            println!("평균: {:.2}", average);
            return; // 함수 종료
        }
    }
    println!("해당 이름의 학생을 찾을 수 없습니다.")
}

// 사용자로부터 입력을 받아 문자열을 반환하는 함수
fn get_input(prompt: &str) -> String {
    let mut input = String::new(); // 입력을 저장할 변수 초기화
    print!("{}", prompt); // 입력 요청 메시지 출력
                          // Rust의 표준 출력(stdout)은 버퍼링(bufferring)을 사용함
                          // 출력 명령이 즉시 화면에 나타나지 않고 내부 버퍼에 저장된 후 한꺼번에 출력되는 방식
                          // print! 매크로는 개행 문자가 없는 출력시에도 버퍼에 저장됨
                          // 따라서 사용자가 입력을 기다리는 상황에서 출력이 즉시 보이지 않을 수 있기 때문에
                          // 아래 라인을 추가함
    io::stdout().flush().unwrap(); // 출력 버퍼 플러시
                                   // flush 메서드: 출력 버퍼에 저장된 모든 데이터를 즉시 실제 출력장치로 전송
                                   // 반환값은 Result<(), std::io::Error> 형태로, 성공시 Ok(()), 실패시 Err 반환
                                   // unwrap 메서드: Result 타입의 값을 해제해 내부의 값을 추출
                                   // flush 메서드가 반환한 Result 값을 처리할 때, 간단하게 성공(Ok)시 내부 값을 반환하고
                                   // 실패(Err)시엔 프로그램을 패닉(panic) 상태로 만듬
                                   // 실제 애플리케이션에서는 unwrap을 사용하는 대신 적절한 오류 처리를 하는게 좋음
                                   // unwrap은 에러가 발생할 경우 프로그램이 즉시 종료되기 때문임
    io::stdin().read_line(&mut input).expect("입력 오류");
    input.trim().to_string() // 입력받은 문자열을 반환(return)
}

// 사용자로부터 점수를 입력받고 유효성을 검사하는 함수
fn get_score(subject: &str) -> i32 {
    loop {
        // 점수 입력 요청
        // format!() 매크로와 format() 함수의 차이점
        // format!() 매크로는 포맷된 문자열을 생성해 String 타입으로 반환
        // 직관적인 문법으로 쉽게 문자열을 포맷할 수 있음
        // 컴파일 타임에 포맷 문자열을 처리해 성능 최적화 가능
        // format() 함수는 std::fmt::format으로, 내부적으로 fmt::Arguments를 사용해 문자열을 포맷함
        // 이 함수는 매크로가 아니며, 매크로에서 생성된 fmt::Arguments를 인자로 받음
        // 직접 fmt::Arguments를 생성해 format() 함수에 전달해야 함
        // 저수준의 포맷팅 작업을 수행할 때 유용할 수 있음
        // 커스텀 포맷팅 로직을 구현할 때 사용
        let input = get_input(&format!("{} 점수를 입력하세요 (0-100): ", subject));
        // 입력을 정수로 파싱 시도
        match input.parse::<i32>() {
            // 유효한 정수인 경우 반환
            Ok(num) if num >= 0 && num <= 100 => return num,
            // 유효하지 않은 경우 오류 메시지 출력
            _ => println!("유효한 점수를 입력해주세요."),
        }
    }
}

pub fn main() {
    let mut students: Vec<Student> = Vec::new(); // 학생들을 저장할 벡터 초기화

    loop {
        println!("\n=== 학생 성적 관리 시스템 ===");
        println!("1. 학생 추가");
        println!("2. 학생 검색");
        println!("3. 종료");
        let choice = get_input("원하는 옵션을 선택하세요: ");

        // as_str() 메서드는 String 타입을 슬라이스(&str)로 변환할 때 사용
        // String 객체가 소유하고 있는 데이터에 대한 불변 참조를 반환
        // String과 &str간의 유연한 변환이 가능
        match choice.as_str() {
            "1" => {
                // 학생 추가 선택시
                let name = get_input("학생 이름 입력: ");
                let math = get_score("수학");
                let science = get_score("과학");
                let english = get_score("영어");

                students.push(Student {
                    name,
                    math,
                    science,
                    english,
                });

                println!("학생이 추가되었습니다.")
            }
            "2" => {
                // 학생 검색 선택시
                let name = get_input("검색할 학생 이름 입력: ");
                search_student(&students, &name);
            }
            "3" => {
                // 종료 선택시
                println!("프로그램을 종료합니다.");
                break;
            }
            _ => {
                // 잘못된 입력시
                println!("유효한 옵션을 선택해주세요.");
            }
        }
    }
}
