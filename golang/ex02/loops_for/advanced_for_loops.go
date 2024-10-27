package loops_for

import (
	"fmt"
)

func basicForLoop() {
	// 기본 for문
	// 초기화문; 조건문; 후처리문 구조
	for i := 0; i < 5; i++ {
		fmt.Printf("기본 for 루프: i = %d\n", i)
	}
}

// 조건에 따른 반복 (조건문만 사용)
func conditionalLoop() {
	count := 0
	for count < 3 {
		fmt.Printf("조건부 반복: count = %d\n", count)
		count++
	}
}

// range 키워드를 사용하는 for 루프
func rangeLoop() {
	// 슬라이스나 배열 등의 요소를 반복할 때 사용하는 range 루프
	elements := []string{"하나", "둘", "셋"}
	// 슬라이스의 모든 요소에 대해 반복 실행
	for index, value := range elements {
		fmt.Printf("index %d: value %s\n", index, value)
	}
}

// 무한 루프
func infiniteLoop() {
	// 명시적으로 종료 조건을 설정하거나 break 키워드로 종료해야 함
	fmt.Println("무한 루프 시작 (5번 실행후 종료)")
	iteration := 0
	for {
		fmt.Printf("무한 루프 반복 횟수: %d\n", iteration)
		iteration++
		if iteration >= 5 {
			fmt.Println("무한 루프 종료")
			break
		}
	}
}

func advancedForLoops() {
	basicForLoop()
	conditionalLoop()
	rangeLoop()
	infiniteLoop()
}
