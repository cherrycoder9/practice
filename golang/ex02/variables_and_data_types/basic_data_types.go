package variables_and_data_types

import "fmt"

func basicDataTypes() {
	// int는 운영체제에 따라 32비트 또는 64비트로 설정됨
	var age int = 38
	fmt.Println("나이: ", age)

	// float32는 소수점 이하 약 7자리까지의 정확도, float64는 15자리
	var height float64 = 173.5
	fmt.Println("키: ", height)

	// 문자열은 쌍따옴표로 묶음, Go에선 기본적으로 UTF-8을 지원
	var name string = "제로디"
	fmt.Println("이름: ", name)

	// bool 타입은 true 또는 false의 값을 가짐
	var isStudent bool = false
	fmt.Println("학생 여부: ", isStudent)

	// 바이트 타입 (uint8)
	// byte는 uint8의 별칭이며, 주로 파일 처리나 바이너리 데이터를 다룰 때 사용함
	var b byte = 255
	fmt.Println("바이트 값: ", b)

	// 룬 타입 (rune)
	// rune은 int32의 별칭이며, 주로 유니코드 문자를 다룰 때 사용함
	var r rune = '가'
	fmt.Println("rune 값: ", r) // 44032

}
