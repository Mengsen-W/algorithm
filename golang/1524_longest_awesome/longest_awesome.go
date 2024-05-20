/*
 * @Date: 2024-05-20
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-05-20
 * @FilePath: /algorithm/golang/1524_longest_awesome/longest_awesome.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func longestAwesome(s string) int {
	prefix := map[int]int{0: -1}
	ans := 0
	sequence := 0
	for j := 0; j < len(s); j++ {
		digit := int(s[j] - '0')
		sequence ^= (1 << digit)
		if prevIndex, ok := prefix[sequence]; ok {
			ans = max(ans, j-prevIndex)
		} else {
			prefix[sequence] = j
		}
		for k := 0; k < 10; k++ {
			if prevIndex, ok := prefix[sequence^(1<<k)]; ok {
				ans = max(ans, j-prevIndex)
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
		{"3242415", 5},
		{"12345678", 1},
		{"213123", 6},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, longestAwesome(test.s), index)
	}
}
