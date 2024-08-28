// 다중 트레이트 경합 해결
// Loggable과 Persistable 두개의 트레이트를 정의
// 각각 log()와 save() 메서드를 선언
// AdvancedLogger란 구조체를 정의하고
// 이 구조체에 대해 두 트레이트를 모두 구현
// 두 트레이트 모두 log() 메서드를 필요로 하고, 구현 방법이 다름
// 따라서 메서드 간의 충돌을 해결해야 함

pub fn main() {}
