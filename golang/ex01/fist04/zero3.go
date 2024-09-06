package main

import "fmt"

// 구조체를 포인터로 전달하면 성능 이점이 있음
// 특히 큰 구조체를 처리할 때 포인터를 사용하면 불필요한 복사 방지
// 포인터를 사용하면 구조체의 필드를 직접 수정할수도 있음

// Person 구조체
type Person3 struct {
	Name string
	Age  int
}

// 구조체를 포인터로 전달하는 함수
func IncreaseAge(p *Person3) {
	p.Age += 1 // 포인터를 사용해 Age 필드 직접 수정
}

func Zero3() {
	p1 := Person3{Name: "민철", Age: 13}

	// 함수 호출 전
	fmt.Println("나이(변경전): ", p1.Age)

	// 포인터를 구조체로 전달해 값 수정
	IncreaseAge(&p1)

	// 함수 호출 후
	fmt.Println("나이(변경후): ", p1.Age)
}
