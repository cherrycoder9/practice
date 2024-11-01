package main

import "fmt"

// Vehicle 인터페이스 정의
// 다양한 타입의 탈것들에 공통적 행동을 정의
type Vehicle interface {
	Move() // 이동 메서드 정의
	Stop() // 정지 메서드 정의
}

// Car 구조체 정의
// 자동차에 대한 필드와 메서드를 포함
type Car struct {
	brand string
}

// Car 구조체의 Move 메서드 구현
func (c Car) Move() {
	fmt.Printf("%s 자동차가 이동합니다.\n", c.brand)
}

// Car 구조체의 Stop 메서드 구현
func (c Car) Stop() {
	fmt.Printf("%s 자동차가 정지합니다.\n", c.brand)
}

// Bicycle 구조체 정의
type Bicycle struct {
	brand string
}

// Bicycle 구조체의 Move 메서드 구현
func (b Bicycle) Move() {
	fmt.Printf("%s 자전거가 이동합니다\n", b.brand)
}

// Bicycle 구조체의 Stop 메서드 구현
func (b Bicycle) Stop() {
	fmt.Printf("%s 자전거가 정지합니다.\n", b.brand)
}

// 인터페이스를 통한 다형성 구현
// 다양한 탈것을 움직이게 하는 함수 정의
func OperateVehicle(v Vehicle) {
	v.Move()
	v.Stop()
}

// 빈 인터페이스를 사용해 모든 타입의 값을 받을 수 있는 함수 정의 가능
func Describe(i interface{}) {
	// 타입 단언(type assertion)을 사용해 i의 실제 타입 확인
	switch v := i.(type) {
	case string:
		fmt.Printf("문자열임, 내용: %s\n", v)
	case int:
		fmt.Printf("정수임, 값: %d\n", v)
	case Vehicle:
		fmt.Printf("Vehicle 인터페이스를 구현한 타입임\n")
		v.Move()
	default:
		fmt.Printf("알 수 없는 타입\n")
	}
}

func vehicleInterfaces() {
	// Car, Bicycle 인스턴스 생성
	car := Car{brand: "현대"}
	bicycle := Bicycle{brand: "삼천리"}

	// Vehicle 인터페이스를 통한 다형성 구현
	fmt.Println("== 다형성을 이용한 Vehicle 조작 ==")
	OperateVehicle(car)     // Car 타입의 인스턴스 사용
	OperateVehicle(bicycle) // Bicycle 타입의 인스턴스 사용

	// 빈 인터페이스를 통한 유연한 함수 사용 예시
	fmt.Println("\n== 빈 인터페이스 사용 예시 ==")
	Describe("안녕하세요") // 문자열 타입
	Describe(42)      // 정수 타입
	Describe(car)     // Vehicle 인터페이스 구현 타입

	// 빈 인터페이스(interface{})를 사용할 때는 런타임에 실제 타입을 검사하므로 성능에 영향을 줄 수 있음
}
