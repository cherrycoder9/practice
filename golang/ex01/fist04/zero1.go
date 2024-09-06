package main

import "fmt"

// Go언어의 구조체는 클래스와 유사하지만 상속 기능은 없음
// type 키워드를 사용해 정의함, 여러 필드를 포함할 수 있음

// Person 구조체 정의
// 필드 변수 이름을 대문자로 하면 패키지 외부에서 접근할 수 있음 (public)
// 소문자로 하면 패키지 내부에서만 접근 가능 (private)
type Person struct {
	Name   string  // 이름 필드
	Age    int     // 나이 필드
	Weight float64 // 체중 필드
}

func Zero1() {
	// 구조체 초기화 및 값 설정
	p1 := Person{Name: "철수", Age: 30, Weight: 60.5}

	// 구조체 필드에 접근
	fmt.Println("이름: ", p1.Name)
	fmt.Println("나이: ", p1.Age)
	fmt.Println("체중: ", p1.Weight)
}
