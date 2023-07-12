/*
 * @Date: 2023-07-12
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-07-12
 * @FilePath: /algorithm/golang/2544_alternate_digit_sum/alternate_digit_sum.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func alternateDigitSum(n int) int {
	res, sign := 0, 1
	for n > 0 {
		res += n % 10 * sign
		sign = -sign
		n /= 10
	}
	return -sign * res
}

func main() {
	tests := []struct {
		n   int
		ans int
	}{{521, 4}, {111, 1}, {886996, 0}}
	for _, item := range tests {
		assert.Equal(&testing.T{}, alternateDigitSum(item.n), item.ans)
	}
}
