// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func numSteps(s string) int {
	n := len(s)
	ans := 0
	// meet1 记录我们有没有遇见过字符 1
	meet1 := false
	// 从后向前遍历字符
	for i := n - 1; i >= 0; i-- {
		if s[i] == '0' {
			if meet1 {
				ans += 2
			} else {
				ans += 1
			}
		} else {
			if !meet1 {
				if i != 0 {
					ans += 2
				}
				meet1 = true
			} else {
				ans++
			}
		}
	}
	return ans
}

func main() {
	tests := []struct {
		s   string
		ans int
	}{
		{"1101", 6},
		{"10", 1},
		{"1", 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, numSteps(test.s), index)
	}
}
