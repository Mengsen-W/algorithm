/*
 * @Date: 2023-10-26
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-26
 * @FilePath: /algorithm/golang/2520_count_digits/count_digits.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func countDigits(num int) int {
	t, res := num, 0
	for t != 0 {
		if num%(t%10) == 0 {
			res += 1
		}
		t /= 10
	}
	return res
}

func main() {
	tests := []struct {
		num int
		ans int
	}{
		{7, 1},
		{121, 2},
		{1248, 4},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, countDigits(test.num), index)
	}
}
