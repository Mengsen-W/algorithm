/*
 * @Date: 2024-05-18
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-05-18
 * @FilePath: /algorithm/golang/2644_max_div_score/max_div_score.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func maxDivScore(nums []int, divisors []int) int {
	cnt := -1
	ans := 0

	for _, divisor := range divisors {
		tmp := 0
		for _, num := range nums {
			if num%divisor == 0 {
				tmp++
			}
		}

		if tmp > cnt || (tmp == cnt && divisor < ans) {
			cnt = tmp
			ans = divisor
		}
	}
	return ans
}

func main() {
	tests := []struct {
		nums     []int
		divisors []int
		ans      int
	}{
		{[]int{4, 7, 9, 3, 9}, []int{5, 2, 3}, 3},
		{[]int{20, 14, 21, 10}, []int{5, 7, 5}, 5},
		{[]int{12}, []int{10, 16}, 10},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maxDivScore(test.nums, test.divisors), index)
	}
}
