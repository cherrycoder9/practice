package main

import (
	"encoding/json"
	"fmt"
)

// 구조체 Product를 정의하고, 필드로 이름, 가격, 카테고리를 포함
// Category는 구조체로 정의하고, 필드로 카테고리명, 카테고리 설명을 포함
// Product 구조체를 JSON 형식으로 변환하는 함수와 JSON 문자열을
// Product 구조체로 변환하는 함수를 작성할 것

// Category 구조체 정의
// 제품의 카테고리 정볼르 저장하는 구조체
type Category struct {
	Name        string `json:"name"`        // 카테고리 이름, JSON 으로 변환될 때 "name" 키로 매핑됨
	Description string `json:"description"` // 카테고리 설명, JSON 으로 변환될 때 "description" 키로 매핑됨
}

// Product 구조체 정의
// 제품 정보를 저장하는 구조체
type Product struct {
	Name  string  `json:"name"`  // 제품 이름, JSON으로 변환될 때 "name" 키로 매핑됨
	Price float64 `json:"price"` // 제품 가격, JSON으로 변환될 때 "price" 키로 매핑됨
	// 명시적 필드 선언
	Category Category `json:"category"` // 제품의 카테고리 정보, JSON 으로 변환될 때 "category" 키로 매핑됨
}

// ToJSON 함수
// Product 구조체를 입력받아 JSON 문자열로 변환해 반환하는 함수
func ToJSON(p Product) string {
	// json.Marshal() 함수는 구조체를 JSON으로 인코딩
	// 이 함수는 JSON 형식의 바이트 슬라이스를 반환, 에러도 반환하므로 에러 처리를 해야함
	jsonData, err := json.Marshal(p) // Product 구조체를 JSON 형식으로 변환
	if err != nil {
		// 변환 중 에러가 발생하면 에러 메시지를 출려갛고 빈 문자열 반환
		fmt.Println("JSON 변환 실패: ", err)
		return ""
	}
	// JSON 형식의 바이트 슬라이스를 문자열로 변환해 반환
	return string(jsonData)
}

// FromJSON 함수
// JSON 문자열을 입력받아 Product 구조체로 변환해 반환하는 함수
func FromJSON(jsonStr string) Product {
	var p Product // Product 구조체를 저장할 변수를 선언
	// json.Unmarshal() 함수는 JSON 문자열을 구조체로 변환함
	// 첫 번째 인자는 JSON 형식의 바이트 슬라이스, 두 번째 인자는 데이터를 저장할 포인터
	err := json.Unmarshal([]byte(jsonStr), &p) // JSON 문자열을 Product 구조체로 변환
	if err != nil {
		// 변환 중 에러가 발생하면 에러 메시지 출력
		fmt.Println("JSON 파싱 실패: ", err)
	}
	// 변환된 Product 구조체를 반환
	return p
}

func Zero7() {
	// Category 구조체 생성
	category := Category{
		Name:        "전자제품",         // 카테고리 이름 설정
		Description: "최신 전자 기기와 장비", // 카테고리 설명 설정
	}

	// Product 구조체 생성
	product := Product{
		Name:     "노트북",    // 제품 이름 설정
		Price:    1500.99,  // 제품 가격 설정
		Category: category, // 제품 카테고리 설정
	}

	// Product 구조체를 JSON 문자열로 변환
	jsonStr := ToJSON(product)
	fmt.Println("JSON 출력: ", jsonStr) // 변환된 JSON 문자열 출력

	// JSON 문자열을 다시 Product 구조체로 변환
	parsedProduct := FromJSON(jsonStr)
	fmt.Println("파싱된 제품 이름: ", parsedProduct.Name)
	fmt.Println("파싱된 제품 가격: ", parsedProduct.Price)
	// 명시적 필드 선언을 했었기 때문에 하위 필드를 모두 작성해야 함
	fmt.Println("파싱된 카테고리 이름: ", parsedProduct.Category.Name)
}
