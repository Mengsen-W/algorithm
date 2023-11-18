/*
 * @Date: 2023-11-18
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-11-18
 * @FilePath: /algorithm/golang/2342_maximum_sum/maximum_sum.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func maximumSum(nums []int) int {
	ans := -1
	mx := [82]int{}
	for _, num := range nums {
		s := 0
		for x := num; x > 0; x /= 10 {
			s += x % 10
		}
		if mx[s] > 0 {
			ans = max(ans, mx[s]+num)
		}
		mx[s] = max(mx[s], num)
	}
	return ans
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{18, 43, 36, 13, 7}, 54},
		{[]int{10, 12, 19, 14}, -1},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maximumSum(test.nums), index)
	}
}
