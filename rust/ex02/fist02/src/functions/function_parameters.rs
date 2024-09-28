// 러스트에선 매개변수가 기본적으로 불변으로 전달됨
// 가변으로 전달하려면 mut 키워드를 사용해야 함
pub fn main() {
    let name = String::from("러스트");
    let age = 30;

    introduce(&name, age);
}

fn introduce(name: &String, age: u32) {
    println!("이름: {}, 나이: {}", name, age);
}
