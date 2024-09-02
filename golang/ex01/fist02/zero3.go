package main

import "fmt"

func Zero3() {
	// 레이블과 break 사용
outerLoop:
	for i := 1; i <= 3; i++ {
		for j := 1; j <= 3; j++ {
			if i*j > 4 {
				// 레이블이 붙은 외부 루프 종료
				break outerLoop
			}
			fmt.Printf("%d * %d = %d\n", i, j, i*j)
		}
	}

	// 레이블과 continue
outerLoop2:
	for i := 1; i <= 3; i++ {
		for j := 1; j <= 3; j++ {
			if i == 2 {
				// 레이블이 붙은 외부 루프의 다음 반복으로 건너뜀
				continue outerLoop2
			}
			fmt.Printf("i = %d, j = %d\n", i, j)
		}
	}
}
