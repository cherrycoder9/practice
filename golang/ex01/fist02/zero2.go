package main

import "fmt"

func Zero2() {
	// 1부터 5까지 반복문을 이용한 출력
	for i := 1; i <= 5; i++ {
		fmt.Println(i)
	}

	// 조건식만 사용한 무한 루프
	n := 0
	for n < 5 {
		fmt.Println(n)
		n++
	}

	// 슬라이스에서 각 값 반복
	// Go언어에서 슬라이스란 동적 배열이라 생각하면 됨
	numbers := []int{10, 20, 30, 40, 50}
	// range를 사용해 numbers 슬라이스의 각 요소 접근
	for index, value := range numbers {
		fmt.Printf("인덱스 %d의 값: %d\n", index, value)
	}

	// 중첩 반복문과 break 구문
	for i := 1; i <= 3; i++ {
		for j := 1; j <= 3; j++ {
			if i*j > 4 {
				// 내부 루프 종료
				break
			}
			fmt.Printf("%d * %d = %d\n", i, j, i*j)
		}
	}

	// continue 구문
	for i := 1; i <= 5; i++ {
		if i%2 == 0 {
			// 짝수이면 다음 반복으로 건너뜀
			continue
		}
		fmt.Println(i)
	}
}
