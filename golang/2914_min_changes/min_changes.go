// Package main ...
package main

import "fmt"

func minChanges(s string) int {
	ans := 0
	for i := 0; i < len(s); i += 2 {
		if s[i] != s[i+1] {
			ans++
		}
	}
	return ans
}

func main() {
	tests := []struct {
		s   string
		ans int
	}{
		{"1001", 2},
		{"10", 1},
		{"0000", 0},
	}

	for index, test := range tests {
		if ans := minChanges(test.s); ans != test.ans {
			panic(fmt.Sprintf("test %d failed", index))
		}
	}
}
