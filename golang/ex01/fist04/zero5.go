package main

import "fmt"

// Person 구조체
type Person5 struct {
	Name string
	Age  int
}

// Person 구조체의 메서드
func (p Person5) Greet() {
	fmt.Println("제 이름은, ", p.Name, "입니다.")
}

// 포인터 리시버를 사용한 메서드 (값 수정 가능)
func (p *Person5) HaveBirthday() {
	p.Age += 1
}

func Zero5() {
	p1 := Person5{Name: "아담", Age: 29}

	// 메서드 호출
	p1.Greet()

	fmt.Println("나이: ", p1.Age)
	// 생일 맞이 후 나이 증가
	p1.HaveBirthday()
	fmt.Println("나이: ", p1.Age)
}
