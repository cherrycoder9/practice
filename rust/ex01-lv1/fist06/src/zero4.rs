// 디폴트 메서드 구현
// Traits에서는 메서드의 디폴트 구현 가능함
// 트레이트를 구현하는 타입에서 특정 메서드를 오버라이드하지 않아도 기본 동작을 가질 수 있음
trait Describable {
    fn describe(&self) -> String {
        String::from("A describable item")
    }
}

// 기본 메서드를 사용하는 타입
struct Item;

impl Describable for Item {}

// 기본 메서드를 오버라이드하는 타입
struct SpecialItem;

impl Describable for SpecialItem {
    fn describe(&self) -> String {
        String::from("A special item!")
    }
}
pub fn main() {
    let item = Item;
    let special_item = SpecialItem;
    println!("{}", item.describe());
    println!("{}", special_item.describe());
}
