package methods_and_receivers

import "fmt"

// 리시버는 구조체에 메서드를 연결하기 위해 사용되는 특별한 매개변수
// 리시버는 함수와 구조체를 연결해 객체 지향 프로그래밍의 메서드와 유사한 기능
// 리시버는 메서드가 호출되는 대상을 가리키며, 보통 구조체의 인스턴스를 의미함
// Go 언어에서는 리시버를 통해 구조체에 메서드를 추가할 수 있으며,
// 리시버의 종류에 따라 값을 복사하거나 참조로 사용할 수 있음

// Person 구조체 정의
type Person struct {
	name string
	age  int
}

// (값 리시버) Person 구조체의 메서드: getName
// 값을 리시버로 사용하는 메서드는 원본 데이터를 변경하지 않음
func (p Person) getName() string {
	return p.name
}

// (포인터 리시버) Person 구조체의 메서드: setName
// 포인터 리시버를 사용해 구조체의 필드를 변경할 수 있음
// 포인터를 사용하지 않으면 메서드 내에서만 변경 사항이 적용됨
func (p *Person) setName(newName string) {
	p.name = newName
}

// (값 리시버) Person 구조체의 메서드: getAge
// 나이를 반환하는 메서드, 리시버는 값을 사용
func (p Person) getAge() int {
	return p.age
}

// (포인터 리시버) Person 구조체의 메서드: setAge
// 나이를 설정하는 메서드, 포인터 리시버를 사용해 실제 값 변경
func (p *Person) setAge(newAge int) {
	p.age = newAge
}

func methodWithReceiver() {
	// person 객체 생성
	person := Person{name: "홍길동", age: 30}

	// getName 메서드를 호출해 이름 출력
	fmt.Println("현재 이름:", person.getName())

	// setName 메서드를 호출해 이름 변경
	person.setName("김철수")
	fmt.Println("변경된 이름:", person.getName())

	// getAge 메서드를 호출해 나이 출력
	fmt.Println("현재 나이:", person.getAge())

	// setAge 메서드를 호출해 나이 변경
	person.setAge(35)
	fmt.Println("변경된 나이:", person.getAge())

	// 리시버 사용 방식은 객체 지향 언어의 this 또는 self와 유사하지만,
	// Go에서는 명시적으로 포인터를 사용함으로써 메모리 관리와 성능을 최적화할 수 있음
}
