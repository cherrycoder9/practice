/// Option 열거형의 구조를 보여주는 예제 함수
fn find_word(word: &str, text: &str) -> Option<usize> {
    text.find(word) // 단어의 위치를 찾으면 Some(인덱스), 없으면 None 반환
}

pub fn main() {
    let text = "러스트 프로그래밍 언어";
    let word = "프로그래밍";

    match find_word(word, text) {
        Some(index) => println!("'{}' 단어의 위치: {}", word, index),
        None => println!("'{}' 단어를 찾을 수 없습니다.", word),
    }
}
