/*
 * @Date: 2023-11-08
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-11-08
 * @FilePath: /algorithm/golang/2609_find_the_longest_balanced_substring/find_the_longest_balanced_substring.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func findTheLongestBalancedSubstring(s string) int {
	res := 0
	count := make([]int, 2)
	for i := 0; i < len(s); i++ {
		if s[i] == '1' {
			count[1]++
			res = max(res, 2*min(count[0], count[1]))
		} else if i == 0 || s[i-1] == '1' {
			count[0], count[1] = 1, 0
		} else {
			count[0]++
		}
	}
	return res
}

func main() {
	tests := []struct {
		s   string
		ans int
	}{
		{"01000111", 6},
		{"00111", 4},
		{"111", 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, findTheLongestBalancedSubstring(test.s), index)
	}
}
