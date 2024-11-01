package main

import "fmt"

// 빈 인터페이스는 모든 타입을 수용할 수 있음
// 여러 타입을 처리해야 하는 상황에서 사용됨
type Any interface{}

// 다양한 타입에 대한 처리를 위해 타입 스위치를 사용한 함수 정의
func handleType(value Any) {
	// 타입 스위치를 사용해 입력된 값의 타입에 따라 다른 로직 수행
	switch v := value.(type) {
	case int:
		fmt.Printf("정수 타입: 값은 %d\n", v)
	case string:
		fmt.Printf("정수 타입: 값은 '%s'\n", v)
	case bool:
		fmt.Printf("정수 타입: 값은 %t\n", v)
	case float64:
		fmt.Printf("정수 타입: 값은 %f\n", v)
	default:
		fmt.Println("알 수 없는 타입")
	}
}

// 다양한 구조체 타입에 따라 처리하는 로직
// 동물을 나타내는 인터페이스 정의
type Animal interface {
	Speak() string
}

// Dog 구조체 정의
type Dog struct{}

func (d Dog) Speak() string {
	return "멍멍"
}

// Cat 구조체 정의
type Cat struct{}

func (c Cat) Speak() string {
	return "야옹"
}

// 타입 스위치를 사용해 다양한 Animal 타입에 대한 로직 처리
func describeAnimal(a Animal) {
	switch animal := a.(type) {
	case Dog:
		fmt.Printf("개입니다: %s\n", animal.Speak())
	case Cat:
		fmt.Printf("고양이입니다: %s\n", animal.Speak())
	default:
		fmt.Println("알 수 없는 동물입니다.")
	}
}

func typeSwitch() {
	// 다양한 타입의 값들을 처리하기 위해 handleType 함수 호출
	handleType(42)
	handleType("안녕")
	handleType(true)
	handleType(3.14)
	handleType([]int{1, 2, 3}) // 디폴트 처리 (알 수 없는 타입)

	// 다형성을 사용해 애니멀 인터페이스를 구현한 구조체 처리
	var a1 Animal = Dog{}
	var a2 Animal = Cat{}
	describeAnimal(a1)
	describeAnimal(a2)
}
