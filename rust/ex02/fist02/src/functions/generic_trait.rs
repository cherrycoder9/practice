// 트레이트 정의
trait Summary {
    fn summarize(&self) -> String;
}

// 구조체 정의
struct Article {
    title: String,
    content: String,
}

struct Tweet {
    username: String,
    content: String,
}

// 트레이트 구현
impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}: {}", self.title, self.content)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, self.content)
    }
}

// 제네릭 함수 정의
fn notify<T: Summary>(item: T) {
    println!("알림: {}", item.summarize());
}

pub fn main() {
    let article = Article {
        title: String::from("Rust 프로그래밍"),
        content: String::from("Rust는 시스템 프로그래밍 언어입니다."),
    };

    let tweet = Tweet {
        username: String::from("rustacean"),
        content: String::from("러스트 러스트"),
    };

    notify(article);
    notify(tweet);
}
