package main

import (
	"fmt"
	"strings"
)

func main() {
	text := "apple,orange,banana"
	separator := ","

	// 문자열 분리
	parts := strings.Split(text, separator)

	fmt.Println(parts) // [apple orange banana]
}
