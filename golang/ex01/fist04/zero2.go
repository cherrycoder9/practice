package main

import (
	"encoding/json"
	"fmt"
)

// 구조체의 필드는 '태그'를 가질 수 있음
// 주로 JSON과 같은 데이터 형식으로 변환할 때 사용함
// 태그는 특정 라이브러리나 도구에 의해 해석됨

// JSON 태그가 포함된 구조체
type Person2 struct {
	Name   string  `json:"name"`   // JSON 필드 이름을 "name"으로 설정
	Age    int     `json:"age"`    // JSON 필드 이름을 "age"로 설정
	Weight float64 `json:"weight"` // JSON 필드 이름을 "weight"로 설정
}

func Zero2() {
	// 구조체 인스턴스 생성
	p1 := Person2{Name: "재호", Age: 25, Weight: 72.2}

	// 구조체를 JSON으로 변환
	// Marshal 함수는 구조체나 맵 같은 데이터를 JSON 형식으로 인코딩
	// GO의 데이터를 JSON 문자열로 변환해주는 함수
	// 반환 값으로는 byte slice와 에러 값을 돌려줌
	// p1이라는 구조체를 JSON 형식으로 변환해 data에 저장, _는 에러 값을 무시하는 구문
	data, _ := json.Marshal(p1)
	// fmt.Println(data)         // 바이트 값이 나옴
	// byte slice를 문자열로 변환
	fmt.Println(string(data)) // {"name":"재호","age":25,"weight":72.2}
}
