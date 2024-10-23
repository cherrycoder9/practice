package variables_and_data_types

import (
	"fmt"
	"strconv"
)

func typeConversion() {
	// 문자열을 정수로 변환
	strToIntExam()

	// 정수를 문자열로 변환
	intToStrExam()

	// 부동소수점 숫자를 문자열로 변환
	floatToStrExam()

	// 문자열을 부동소수점 숫자로 변환
	strToFloatExam()
}

func strToIntExam() {
	// 문자열 "123"을 정수로 변환함
	// strconv 패키지의 Atoi 메서드 사용
	str := "123"
	intValue, err := strconv.Atoi(str)
	if err != nil {
		// 변환 도중 오류가 발생하면 오류 메시지 출력
		fmt.Println("문자열을 정수로 변환할 수 없습니다: ", err)
		return
	}
	// 변환 결과 출력
	fmt.Printf("문자열 \"%s\"를 정수 %d로 변환했습니다.\n", str, intValue)
}

func intToStrExam() {
	// 정수 456을 문자열로 변환
	// strconv 패키지의 Itoa 메서드 사용
	intVal := 456
	strValue := strconv.Itoa(intVal)
	// 변환 결과 출력
	fmt.Printf("정수 %d를 문자열 \"%s\"로 변환했습니다.\n", intVal, strValue)
}

func floatToStrExam() {
	// 부동소수점 숫자 123.456을 문자열로 변환
	// strconv 패키지의 FormatFloat 메서드를 사용
	floatVal := 123.456
	// f: 고정소수점 형식 사용, 2: 소수점 이하 두 자리, 64: float64 타입을 의미
	strValue := strconv.FormatFloat(floatVal, 'f', 2, 64)
	fmt.Printf("부동 소수점 숫자 %f를 문자열 \"%s\"로 변환 완료\n", floatVal, strValue)
}

func strToFloatExam() {
	// 문자열 "789.101"을 부동소수점 숫자로 변환
	// strconv 패키지의 ParseFloat 메서드를 사용
	str := "789.101"
	floatValue, err := strconv.ParseFloat(str, 64)
	if err != nil {
		fmt.Println("문자열을 부동 소수점 숫자로 변환할 수 없습니다: ", err)
		return
	}
	// 변환 결과 출력
	fmt.Printf("문자열 \"%s\"를 부동소수점 숫자 %f로 변환 완료\n", str, floatValue)
}
