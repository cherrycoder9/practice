package control_statements_conditionals

import "fmt"

func controlStatements() {
	// if-else 조건문
	var age int
	fmt.Print("나이를 입력하세요: ")
	fmt.Scanf("%d", &age)
	fmt.Scanln() // 버퍼에 남아있는 엔터 제거, 하지만 bufio.Reader 사용 권장함

	if age < 18 {
		fmt.Println("미성년자")
	} else if age >= 18 && age < 65 {
		fmt.Println("성인")
	} else {
		fmt.Println("노인")
	}

	// switch 조건문
	var day int
	fmt.Print("요일을 숫자로 입력하세요 (1-7): ")
	fmt.Scanf("%d", &day)
	fmt.Scanln() // 버퍼에 남아있는 엔터 제거

	switch day {
	case 1:
		fmt.Println("월요일")
	case 2:
		fmt.Println("화요일")
	case 3:
		fmt.Println("수요일")
	case 4:
		fmt.Println("목요일")
	case 5:
		fmt.Println("금요일")
	case 6:
		fmt.Println("토요일")
	case 7:
		fmt.Println("일요일")
	default:
		fmt.Println("잘못된 입력. 1에서 7사이의 숫자만")
	}

	// 표현식 없는 switch
	// 기본 if-else 조건문과 유사한 구조
	score := 85
	switch {
	case score >= 90:
		fmt.Println("점수 높음")
	case score >= 70:
		fmt.Println("평균 점수")
	default:
		fmt.Println("노력 필요")
	}

}
