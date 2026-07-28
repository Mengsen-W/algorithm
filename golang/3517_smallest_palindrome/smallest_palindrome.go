// Package main ...
package main

import (
	"fmt"
	"strings"
)

func smallestPalindrome(s string) string {
	partition := len(s) / 2
	bucket := make([]int, 26)

	for i := 0; i < partition; i++ {
		bucket[s[i]-'a'] += 1
	}

	var leftBuilder strings.Builder
	for i := 0; i < 26; i++ {
		if bucket[i] > 0 {
			leftBuilder.WriteString(strings.Repeat(string(rune(i+'a')), bucket[i]))
		}
	}
	left := leftBuilder.String()

	mid := ""
	if len(s)%2 != 0 {
		mid = string(s[partition])
	}

	rightBytes := []byte(left)
	for i, j := 0, len(rightBytes)-1; i < j; i, j = i+1, j-1 {
		rightBytes[i], rightBytes[j] = rightBytes[j], rightBytes[i]
	}
	right := string(rightBytes)

	return left + mid + right
}

func main() {
	tests := []struct {
		s   string
		ans string
	}{
		{"z", "z"},
		{"babab", "abbba"},
		{"daccad", "acddca"},
	}

	for index, test := range tests {
		if smallestPalindrome(test.s) != test.ans {
			fmt.Println(index)
		}
	}
}
