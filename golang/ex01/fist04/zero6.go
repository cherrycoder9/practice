package main

import "fmt"

// 제조사, 모델명, 연식을 가진 Vehicle 구조체 생성
// 구조체 Car에 구조체 Vehicle을 임베딩하고 추가로 좌석수 필드 추가
// 두 구조체에 각각 Description() 메서드 구현
// Vehicle 구조체에서는 기본적인 차량 설명 출력
// Car 구조체에서는 좌석 수까지 포함한 설명 출력
// Car의 Description() 메서드에서 Vehicle의 Description() 메서드를 호출하는 구조

// Vehicle 구조체
type Vehicle struct {
	Make  string // 제조사
	Model string // 모델명
	Year  int    // 연식
}

// Vehicle 구조체에 대한 Description 메서드
func (v Vehicle) Description() string {
	// Sprintf: Go 표준 라이브러리의 fmt 패키지에 포함된 함수
	// 형식화된 문자열을 생성하는 기능
	// 실제 출력은 하지 않고 결과를 문자열로 반환하는 특징
	return fmt.Sprintf("제조사: %s, 모델명: %s, 연식: %d", v.Make, v.Model, v.Year)
}

// Car 구조체
type Car struct {
	// 필드명을 따로 지정하지 않고 구조체 타입만으로 필드가 정의되는 경우를 임베딩이라고 함
	Vehicle     // 익명 필드로 임베딩, Vehicle 구조체의 필드나 메서드에 직접 접근할 수 있게됨
	Seats   int // 좌석수
}

// Car 구조체에 대한 Description 메서드
func (c Car) Description() string {
	// 먼저 Vehicle 구조체의 Description 메서드를 호출해 차량 기본 정보를 가져옴
	vehicleDescription := c.Vehicle.Description()

	// Vehicle 설명에 좌석 수 정보를 추가해 반환
	return fmt.Sprintf("%s, 좌석수: %d", vehicleDescription, c.Seats)
}

func Zero6() {
	// Vehicle 필드와 Car 구조체 필드를 모두 초기화
	car := Car{
		Vehicle: Vehicle{Make: "현대", Model: "소나타", Year: 2023}, // Vehicle 필드 초기화
		Seats:   5,                                             // Car 구조체의 Seats vlfem chrlghk
	}

	fmt.Println("차량 설명: ", car.Description())
}
