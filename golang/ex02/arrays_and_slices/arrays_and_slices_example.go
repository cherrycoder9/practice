package arrays_and_slices

import "fmt"

func arraysAndSlicesExample() {
	// 배열은 고정된 크기를 가지며 선언 시 크기를 지정해야 함
	var numbers [5]int

	// 배열 초기화
	numbers[0] = 10
	numbers[1] = 20
	numbers[2] = 30
	numbers[3] = 40
	numbers[4] = 50

	// 배열 출력
	fmt.Println("배열의 값들: ", numbers) // 배열의 모든 요소 출력

	// 슬라이스는 배열과 다르게 크기가 가변적, 기본적으로 배열의 레퍼런스로 작동
	slice := []int{1, 2, 3, 4, 5} // 슬라이스 리터럴을 사용해 초기화

	// 슬라이스에 값 추가
	slice = append(slice, 6)
	fmt.Println("슬라이스의 값들: ", slice)

	// 슬라이스의 부분 선택
	subSlice := slice[1:4]             // 슬라이스의 1번 인덱스부터 4번 인덱스 전까지 선택
	fmt.Println("부분 슬라이스: ", subSlice) // 선택된 슬라이스 요소 출력

	// 슬라이스 용량(capacity)과 길이(length)
	fmt.Println("슬라이스 길이: ", len(slice))
	fmt.Println("슬라이스 용량: ", cap(slice))

	// 슬라이스는 배열에 대한 레퍼런스로 작동하여, 변경 시 원래 배열도 함께 변경됨
	baseArray := [5]int{100, 200, 300, 400, 500}
	baseSlice := baseArray[1:4] // 배열의 부분을 참조하여 슬라이스 생성
	fmt.Println("기본 배열: ", baseArray)
	fmt.Println("생성된 슬라이스: ", baseSlice)

	// 슬라이스 요소 변경
	baseSlice[0] = 250
	fmt.Println("변경 후 기본 배열: ", baseArray)
	fmt.Println("변경 후 슬라이스: ", baseSlice)

	// make 함수를 사용한 슬라이스 생성
	// 슬라이스의 초기 용량과 길이를 설정할 수 있음
	madeSlice := make([]int, 3, 5) // 길이 3, 용량 5로 슬라이스 생성
	madeSlice[0] = 7
	madeSlice[1] = 8
	madeSlice[2] = 9
	fmt.Println("make 함수로 생성된 슬라이스: ", madeSlice)
	fmt.Println("make 슬라이스 길이: ", len(madeSlice))
	fmt.Println("make 슬라이스 용량: ", cap(madeSlice))

}
