// Package main ...
package main

import "fmt"

func longestValidParentheses(s string) int {
	max := func(x, y int) int {
		if x > y {
			return x
		}
		return y
	}
	left, right, maxLength := 0, 0, 0
	for i := 0; i < len(s); i++ {
		if s[i] == '(' {
			left++
		} else {
			right++
		}
		if left == right {
			maxLength = max(maxLength, 2*right)
		} else if right > left {
			left, right = 0, 0
		}
	}
	left, right = 0, 0
	for i := len(s) - 1; i >= 0; i-- {
		if s[i] == '(' {
			left++
		} else {
			right++
		}
		if left == right {
			maxLength = max(maxLength, 2*left)
		} else if left > right {
			left, right = 0, 0
		}
	}
	return maxLength
}

func main() {
	tests := []struct {
		s   string
		ans int
	}{
		{"(()", 2},
		{")()())", 4},
		{"", 0},
	}

	for _, tt := range tests {
		if got := longestValidParentheses(tt.s); got != tt.ans {
			panic(fmt.Sprintf("longestValidParentheses(%q) = %d, want %d", tt.s, got, tt.ans))
		}
	}
}
