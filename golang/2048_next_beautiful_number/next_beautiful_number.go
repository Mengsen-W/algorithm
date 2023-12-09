/*
 * @Date: 2023-12-09
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-09
 * @FilePath: /algorithm/golang/2048_next_beautiful_number/next_beautiful_number.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func isBalance(x int) bool {
	count := make([]int, 10)
	for x > 0 {
		count[x%10]++
		x /= 10
	}
	for i := 0; i < 10; i++ {
		if count[i] > 0 && count[i] != i {
			return false
		}
	}
	return true
}

func nextBeautifulNumber(n int) int {
	for i := n + 1; i <= 1224444; i++ {
		if isBalance(i) {
			return i
		}
	}
	return -1
}

func main() {
	tests := []struct {
		x   int
		ans int
	}{{1, 22}, {1000, 1333}, {3000, 3133}}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, nextBeautifulNumber(test.x), index)
	}
}
