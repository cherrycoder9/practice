package maps

import "fmt"

func mapOperations() {
	// Go 언어에서 맵(map) 자료구조는 키(key)와 값(value)을 쌍으로 저장하는 해시 테이블 구조
	// 다른 언어의 해시맵과 유사하지만, 타입 안정성이 보장되고 동적 크기 조정이 자동으로 이뤄짐
	// 맵 선언 초기화
	// make 함수를 사용해 맵을 초기화함. 키와 값의 타입을 지정해야 함
	students := make(map[string]int)

	// 학생 이름을 키로, 학번을 값으로 추가
	students["김철수"] = 101
	students["이영희"] = 102
	students["박지훈"] = 103

	// 맵에 값 추가 및 출력 예시
	fmt.Println("학생 명단과 학번: ")
	for name, id := range students {
		fmt.Printf("이름: %s, 학번: %d\n", name, id)
	}

	// 특정 키에 접근
	kimID := students["김철수"]
	fmt.Printf("김철수의 학번: %d\n", kimID)

	// 키 존재 여부 확인
	// Go에서는 맵에서 키에 접근할 때, 두 번째 반환값으로 해당 키의 존재 여부를 확인할 수 있음
	id, exists := students["홍길동"]
	if exists {
		fmt.Printf("홍길동의 학번: %d\n", id)
	} else {
		fmt.Println("홍길동은 명단에 없습니다.")
	}

	// 값 삭제
	// delete 함수를 사용해 맵에서 특정 키 삭제
	delete(students, "박지훈")
	fmt.Println("박지훈 삭제 후 학생 명단:")
	for name, id := range students {
		fmt.Printf("이름: %s, 학번: %d\n", name, id)
	}

	// 맵의 길이 확인
	// len 함수를 사용해 현재 맵에 저장된 키-값 쌍의 개수 확인
	fmt.Printf("현재 학생 수: %d명\n", len(students))
}
