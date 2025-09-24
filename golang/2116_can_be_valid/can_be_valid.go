// Package main ...
package main

import (
	"math"
	"testing"

	"github.com/stretchr/testify/assert"
)

func canBeValid(s string, locked string) bool {
	n := len(s)
	mx := 0 // 可以达到的最大分数
	mn := 0 // 可以达到的最小分数 与 最小有效前缀对应分数 的较大值
	for i := 0; i < n; i++ {
		if locked[i] == '1' {
			// 此时对应字符无法更改
			var diff int
			if s[i] == '(' {
				diff = 1
			} else {
				diff = -1
			}
			mx += diff
			mn = int(math.Max(float64(mn+diff), float64((i+1)%2)))
		} else {
			// 此时对应字符可以更改
			mx++
			mn = int(math.Max(float64(mn-1), float64((i+1)%2)))
		}
		if mx < mn {
			// 此时该前缀无法变为有效前缀
			return false
		}
	}
	// 最终确定 s 能否通过变换使得分数为 0（成为有效字符串）
	return mn == 0
}

func main() {
	tests := []struct {
		s      string
		locked string
		ans    bool
	}{
		{"))()))", "010100", true},
		{"()()", "0000", true},
		{")", "0", false},
		{"(((())(((())", "111111010111", true},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, canBeValid(test.s, test.locked), test.ans, index)
	}
}
