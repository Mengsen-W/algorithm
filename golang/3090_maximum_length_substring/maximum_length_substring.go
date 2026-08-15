// Package main ...
package main

import "fmt"

func maximumLengthSubstring(s string) int {
	count := make([]int, 26)
	left := 0
	res := 0
	for right := 0; right < len(s); right++ {
		ch := s[right] - 'a'
		count[ch]++
		for count[ch] > 2 {
			ch2 := s[left] - 'a'
			count[ch2]--
			left++
		}
		length := right - left + 1
		if length > res {
			res = length
		}
	}
	return res
}

func main() {
	tests := []struct {
		s   string
		ans int
	}{
		{"bcbbbcba", 4},
		{"aaaa", 2},
	}

	for index, test := range tests {
		result := maximumLengthSubstring(test.s)
		if result != test.ans {
			fmt.Printf("Test %d failed: expected %d, got %d\n", index, test.ans, result)
		} else {
			fmt.Printf("Test %d passed\n", index)
		}
	}
}
