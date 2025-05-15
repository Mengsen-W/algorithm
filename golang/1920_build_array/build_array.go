// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func buildArray(nums []int) []int {
	n := len(nums)
	// 第一次遍历编码最终值
	for i := 0; i < n; i++ {
		nums[i] += 1000 * (nums[nums[i]] % 1000)
	}
	// 第二次遍历修改为最终值
	for i := 0; i < n; i++ {
		nums[i] /= 1000
	}
	return nums
}

func main() {
	tests := []struct {
		nums []int
		ans  []int
	}{
		{[]int{0, 2, 1, 5, 3, 4}, []int{0, 1, 2, 4, 5, 3}},
		{[]int{5, 0, 1, 2, 3, 4}, []int{4, 5, 0, 1, 2, 3}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, buildArray(test.nums), "case %d", index)
	}
}
