package main

import "fmt"

// 임베딩: Go에서는 구조체 안에 다른 구조체를 임베딩할 수 있음
// 상속과 유사한 기능 구현 가능
// 임베딩된 구조체의 필드와 메서드는 외부 구조체에서 직접 접근 가능

// Address 구조체 정의
type Address struct {
	City, Country string
}

// Person 구조체에 Address 구조체를 임베딩
type Person4 struct {
	Name    string
	Age     int
	Address // 임베딩된 구조체
}

func Zero4() {
	// Person 구조체 인스턴스 생성
	p1 := Person4{
		Name: "경환",
		Age:  40,
		Address: Address{
			City:    "서울",
			Country: "대한민국",
		},
	}

	// 임베딩된 구조체의 필드에 접근
	fmt.Println("이름: ", p1.Name)
	fmt.Println("나이: ", p1.Age)
	fmt.Println("도시: ", p1.City)
	fmt.Println("국가: ", p1.Country)

}
