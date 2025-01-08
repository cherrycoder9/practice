package main

import (
	"fmt"
	"strings"
)

func main() {
	text := "Hello World!"
	substr := "Wor"

	if strings.Contains(text, substr) {
		fmt.Println("문자열에 포함되어 있습니다.")
	} else {
		fmt.Println("문자열에 포함되어 있지 않습니다.")
	}
}
